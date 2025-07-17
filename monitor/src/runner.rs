use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::{Debug, Display},
    fs::File,
    io::Read,
    net::Ipv4Addr,
    path::Path,
    process::Command,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use cfg_if::cfg_if;
use itertools::Itertools;
use jane_eyre::eyre::{self, bail};
use mktemp::Temp;
use serde::{Deserialize, Serialize};
use settings::profile::ImageType;
use tracing::{error, info, trace, warn};

use crate::{
    data::get_runner_data_path,
    github::{unregister_runner, ApiRunner},
    libvirt::{get_ipv4_address, libvirt_prefix, take_screenshot, update_screenshot},
    LIB_MONITOR_DIR,
};

#[derive(Debug, Serialize)]
pub struct Runners {
    runners: BTreeMap<usize, Runner>,
}

/// State of a runner and its live resources.
#[derive(Debug, Serialize)]
pub struct Runner {
    id: usize,
    created_time: SystemTime,
    registration: Option<ApiRunner>,
    guest_name: Option<String>,
    ipv4_address: Option<Ipv4Addr>,
    github_jitconfig: Option<String>,
    details: RunnerDetails,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct RunnerDetails {
    image_type: ImageType,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Status {
    Invalid,
    StartedOrCrashed,
    Idle,
    Reserved,
    Busy,
    DoneOrUnregistered,
}

impl Runners {
    pub fn new(registrations: Vec<ApiRunner>, guest_names: Vec<String>) -> Self {
        // Gather all known runner ids with live resources.
        let registration_ids = registrations
            .iter()
            .flat_map(|registration| registration.name.rsplit_once('@'))
            .flat_map(|(name, _host)| name.rsplit_once('.'))
            .flat_map(|(_, id)| id.parse())
            .collect::<Vec<usize>>();
        let guest_ids = guest_names
            .iter()
            .flat_map(|guest| guest.rsplit_once('.'))
            .flat_map(|(_, id)| id.parse())
            .collect::<Vec<usize>>();
        let ids: BTreeSet<usize> = registration_ids
            .iter()
            .copied()
            .chain(guest_ids.iter().copied())
            .collect();
        trace!(?ids, ?registration_ids, ?guest_ids);

        // Create a tracking object for each runner id.
        let mut runners = BTreeMap::default();
        for id in ids {
            let runner = match Runner::new(id) {
                Ok(runner) => runner,
                Err(error) => {
                    warn!(
                        runner_id = id,
                        ?error,
                        "Failed to create Runner object: {error}",
                    );
                    continue;
                }
            };
            runners.insert(id, runner);
        }

        // Populate the tracking objects with references to live resources.
        for (id, registration) in registration_ids.iter().zip(registrations) {
            if let Some(runner) = runners.get_mut(id) {
                runner.registration = Some(registration);
            }
        }
        for (id, guest_name) in guest_ids.iter().zip(guest_names) {
            if let Some(runner) = runners.get_mut(id) {
                let ipv4_address = runner_ipv4_address(&guest_name);
                runner.guest_name = Some(guest_name);
                runner.ipv4_address = ipv4_address;
            }
        }

        Self { runners }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&usize, &Runner)> {
        self.runners.iter()
    }

    pub fn get(&self, id: usize) -> Option<&Runner> {
        self.runners.get(&id)
    }

    pub fn by_profile<'s>(&'s self, key: &'s str) -> impl Iterator<Item = (&'s usize, &'s Runner)> {
        self.runners
            .iter()
            .filter(move |(_, runner)| runner.base_vm_name() == key)
    }

    pub fn unregister_runner(&self, id: usize) -> eyre::Result<()> {
        let Some(registration) = self
            .runners
            .get(&id)
            .and_then(|runner| runner.registration())
        else {
            bail!("Tried to unregister an unregistered runner");
        };
        info!(runner_id = id, registration.id, "Unregistering runner");
        unregister_runner(registration.id)?;

        Ok(())
    }

    pub fn reserve_runner(
        &self,
        id: usize,
        unique_id: &str,
        qualified_repo: &str,
        run_id: &str,
    ) -> eyre::Result<()> {
        let Some(runner) = self.runners.get(&id) else {
            bail!("No runner with id exists: {id}");
        };
        let Some(registration) = runner.registration() else {
            bail!("Tried to reserve an unregistered runner");
        };
        info!(runner_id = id, registration.id, "Reserving runner");
        let exit_status = Command::new("./reserve-runner.sh")
            .current_dir(&*LIB_MONITOR_DIR)
            .arg(&registration.id.to_string())
            .arg(unique_id)
            .arg(format!("{qualified_repo}/actions/runs/{run_id}"))
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
        if exit_status.success() {
            Ok(())
        } else {
            eyre::bail!("Command exited with status {}", exit_status);
        }
    }

    pub fn screenshot_runner(&self, id: usize) -> eyre::Result<Temp> {
        let Some(runner) = self.runners.get(&id) else {
            bail!("No runner with id exists: {id}");
        };
        let Some(guest_name) = runner.guest_name.as_deref() else {
            bail!("Tried to screenshot a runner with no libvirt guest: {id}");
        };
        let result = Temp::new_file()?;
        let output_path = result.clone();
        take_screenshot(guest_name, &output_path)?;

        Ok(result)
    }

    pub fn update_screenshots(&self) {
        for &id in self.runners.keys() {
            if let Err(error) = self.update_screenshot(id) {
                error!(id, ?error, "Failed to update screenshot for runner");
            }
        }
    }

    fn update_screenshot(&self, id: usize) -> eyre::Result<()> {
        let Some(runner) = self.runners.get(&id) else {
            bail!("No runner with id exists: {id}");
        };
        let Some(guest_name) = runner.guest_name.as_deref() else {
            bail!("Tried to screenshot a runner with no libvirt guest: {id}");
        };
        let output_dir = get_runner_data_path(id, None)?;
        update_screenshot(guest_name, &output_dir)?;

        Ok(())
    }

    pub fn github_jitconfig(
        &self,
        remote_addr: web::auth::RemoteAddr,
    ) -> eyre::Result<Option<&str>> {
        for (_id, runner) in self.runners.iter() {
            if let Some(ipv4_address) = runner.ipv4_address {
                if remote_addr == ipv4_address {
                    return Ok(runner.github_jitconfig.as_deref());
                }
            }
        }

        bail!("No runner found with IP address: {}", remote_addr)
    }

    pub fn update_ipv4_addresses(&mut self) {
        for (&id, runner) in self.runners.iter_mut() {
            if let Some(guest_name) = runner.guest_name.as_deref() {
                let ipv4_address = get_ipv4_address(guest_name);
                if ipv4_address != runner.ipv4_address {
                    info!(
                        "IPv4 address changed for runner {id}: {:?} -> {:?}",
                        runner.ipv4_address, ipv4_address
                    );
                }
                runner.ipv4_address = ipv4_address;
            }
        }
    }

    pub fn boot_script(&self, remote_addr: web::auth::RemoteAddr) -> eyre::Result<Option<String>> {
        for (&id, runner) in self.runners.iter() {
            if let Some(ipv4_address) = runner.ipv4_address {
                if remote_addr == ipv4_address {
                    let path = get_runner_data_path(id, Path::new("boot-script"))?;
                    let mut result = String::default();
                    File::open(path)?.read_to_string(&mut result)?;
                    return Ok(Some(result));
                }
            }
        }

        Ok(None)
    }
}

impl Runner {
    /// Creates an object for tracking the state of a runner.
    ///
    /// For use by [`Runners::new`] only. Does not create a runner.
    fn new(id: usize) -> eyre::Result<Self> {
        let created_time = runner_created_time(id)?;
        trace!(?created_time, "[{id}]");

        let github_jitconfig = match read_github_jitconfig(id) {
            Ok(result) => Some(result),
            Err(error) => {
                warn!(?error, "Failed to get GitHub jitconfig of runner");
                None
            }
        };

        let details = runner_details(id)?;

        Ok(Self {
            id,
            created_time,
            registration: None,
            guest_name: None,
            ipv4_address: None,
            github_jitconfig: github_jitconfig,
            details,
        })
    }

    pub fn registration(&self) -> Option<&ApiRunner> {
        self.registration.as_ref()
    }

    pub fn log_info(&self) {
        fn fmt_option_display<T: Display>(x: Option<T>) -> String {
            x.map_or("None".to_owned(), |x| format!("{}", x))
        }
        fn fmt_option_debug<T: Debug>(x: Option<T>) -> String {
            x.map_or("None".to_owned(), |x| format!("{:?}", x))
        }
        info!(
            "[{}] profile {}, ipv4 {}, status {:?}, age {}, jitconfig {}, reserved for {}",
            self.id,
            self.base_vm_name(),
            fmt_option_display(self.ipv4_address),
            self.status(),
            fmt_option_debug(self.age().ok()),
            self.github_jitconfig.as_ref().map_or("no", |_| "yes"),
            fmt_option_debug(self.reserved_since().ok().flatten()),
        );
        if let Some(registration) = self.registration() {
            if !registration.labels.is_empty() {
                info!(
                    "[{}] - github labels: {}",
                    self.id,
                    registration.labels().join(","),
                );
            }
            if let Some(workflow_run) = registration.label_with_key("reserved-by") {
                info!(
                    "[{}] - workflow run page: https://github.com/{}",
                    self.id, workflow_run
                );
            }
        }
    }

    pub fn age(&self) -> eyre::Result<Duration> {
        Ok(self.created_time.elapsed()?)
    }

    pub fn reserved_since(&self) -> eyre::Result<Option<Duration>> {
        if let Some(registration) = &self.registration {
            if let Some(reserved_since) = registration.label_with_key("reserved-since") {
                let reserved_since = reserved_since.parse::<u64>()?;
                let reserved_since = UNIX_EPOCH + Duration::from_secs(reserved_since);
                return Ok(Some(reserved_since.elapsed()?));
            }
        }

        Ok(None)
    }

    pub fn status(&self) -> Status {
        if self.guest_name.is_none() {
            return Status::Invalid;
        };
        let Some(registration) = &self.registration else {
            return Status::DoneOrUnregistered;
        };
        if registration.busy {
            return Status::Busy;
        }
        if registration.label_with_key("reserved-for").is_some() {
            return Status::Reserved;
        }
        if registration.status == "online" {
            return Status::Idle;
        }
        return Status::StartedOrCrashed;
    }

    pub fn base_vm_name(&self) -> &str {
        self.base_vm_name_from_registration()
            .or_else(|| self.base_vm_name_from_guest_name())
            .expect("Bug in list_runner_guests() or the call to Runners::new()")
    }

    fn base_vm_name_from_registration(&self) -> Option<&str> {
        self.registration
            .iter()
            .flat_map(|registration| registration.name.rsplit_once('@'))
            .flat_map(|(rest, _host)| rest.rsplit_once('.'))
            .map(|(base, _id)| base)
            .next()
    }

    fn base_vm_name_from_guest_name(&self) -> Option<&str> {
        let prefix = libvirt_prefix();
        self.guest_name
            .iter()
            .flat_map(|name| name.strip_prefix(&prefix))
            .flat_map(|name| name.rsplit_once('.'))
            .map(|(base, _id)| base)
            .next()
    }
}

cfg_if! {
    if #[cfg(not(test))] {
        use crate::github::ApiGenerateJitconfigResponse;

        fn read_github_jitconfig(id: usize) -> eyre::Result<String> {
            let result = get_runner_data_path(id, Path::new("github-api-registration"))?;
            let result: ApiGenerateJitconfigResponse =
                serde_json::from_reader(File::open(result)?)?;
            Ok(result.encoded_jit_config)
        }

        fn runner_created_time(id: usize) -> eyre::Result<SystemTime> {
            let created_time_path = get_runner_data_path(id, Path::new("created-time"))?;
            let runner_toml_path = get_runner_data_path(id, Path::new("runner.toml"))?;
            let result = std::fs::metadata(created_time_path)
                .or_else(|_| std::fs::metadata(&runner_toml_path))?
                .modified()?;

            Ok(result)
        }

        fn runner_details(id: usize) -> eyre::Result<RunnerDetails> {
            let runner_toml_path = get_runner_data_path(id, Path::new("runner.toml"))?;

            if let Ok(mut runner_toml) = File::open(&runner_toml_path) {
                let mut contents = String::default();
                runner_toml.read_to_string(&mut contents)?;
                Ok(toml::from_str(&contents)?)
            } else {
                Ok(RunnerDetails::default())
            }
        }

        fn runner_ipv4_address(guest_name: &String) -> Option<Ipv4Addr> {
            get_ipv4_address(guest_name)
        }
    } else {
        use std::cell::RefCell;

        use jane_eyre::eyre::OptionExt;

        thread_local! {
            static RUNNER_CREATED_TIMES: RefCell<BTreeMap<usize, SystemTime>> = RefCell::new(BTreeMap::new());
        }

        fn read_github_jitconfig(_id: usize) -> eyre::Result<String> {
            Ok("".to_owned())
        }

        fn runner_created_time(id: usize) -> eyre::Result<SystemTime> {
            RUNNER_CREATED_TIMES.with_borrow(|created_times| {
                created_times.get(&id).copied().ok_or_eyre("Failed to check runner created time (fake)")
            })
        }

        pub(crate) fn set_runner_created_time_for_test(id: usize, created_time: impl Into<Option<SystemTime>>) {
            RUNNER_CREATED_TIMES.with_borrow_mut(|created_times| {
                if let Some(created_time) = created_time.into() {
                    created_times.insert(id, created_time);
                } else {
                    created_times.remove(&id);
                }
            });
        }

        fn runner_details(_id: usize) -> eyre::Result<RunnerDetails> {
            Ok(RunnerDetails::default())
        }

        fn runner_ipv4_address(_guest_name: &String) -> Option<Ipv4Addr> {
            None
        }
    }
}
