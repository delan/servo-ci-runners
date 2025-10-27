usage
=====

This program analyses usage efficiency and value for money.

```
$ ./download-logs.sh ci{0..4}.servo.org
$ cargo run -r -- ci{0..4}.servo.org.log
```

## 2025-10-27

```
$ cargo run -r -- ci{0..4}.servo.org.log
```
### ci0.servo.org.log
Over the last PT4359414.12643S (50.46 days) of uptime:
- 647 runners in profile servo-macos13:
    - Busy for 13.18%, PT574580.774981S (6.65 days)
    - DoneOrUnregistered for 0.09%, PT3930.32133S (0.05 days)
    - Idle for 52.37%, PT2283090.212073S (26.42 days)
    - Reserved for 0.34%, PT14667.737668S (0.17 days)
    - StartedOrCrashed for 0.58%, PT25336.974917S (0.29 days)
- 2817 runners in profile servo-ubuntu2204:
    - Busy for 9.41%, PT410177.701741S (4.75 days)
    - DoneOrUnregistered for 0.11%, PT4579.755804S (0.05 days)
    - Idle for 116.50%, PT5078926.710406S (58.78 days)
    - Invalid for 0.00%, PT25.612676S (0.00 days)
    - Reserved for 0.39%, PT16942.403332S (0.20 days)
    - StartedOrCrashed for 5.87%, PT255687.732156S (2.96 days)
- 3740 runners in profile servo-windows10:
    - Busy for 29.57%, PT1288939.629473S (14.92 days)
    - DoneOrUnregistered for 0.24%, PT10333.818407S (0.12 days)
    - Idle for 160.65%, PT7003371.818586S (81.06 days)
    - Invalid for 0.00%, PT126.549871S (0.00 days)
    - Reserved for 1.02%, PT44577.186043S (0.52 days)
    - StartedOrCrashed for 6.66%, PT290504.057738S (3.36 days)
### ci1.servo.org.log
Over the last PT4397509.039447S (50.90 days) of uptime:
- 1411 runners in profile servo-macos13:
    - Busy for 3.64%, PT160073.278362S (1.85 days)
    - DoneOrUnregistered for 0.03%, PT1378.371932S (0.02 days)
    - Idle for 25.95%, PT1140939.748582S (13.21 days)
    - Invalid for 0.00%, PT6.455368S (0.00 days)
    - Reserved for 0.16%, PT6986.619457S (0.08 days)
    - StartedOrCrashed for 3.17%, PT139603.062723S (1.62 days)
- 11 runners in profile servo-macos15:
    - Idle for 66.29%, PT2915017.002033S (33.74 days)
    - StartedOrCrashed for 0.01%, PT440.328884S (0.01 days)
- 4177 runners in profile servo-ubuntu2204:
    - Busy for 11.13%, PT489314.68796S (5.66 days)
    - DoneOrUnregistered for 0.14%, PT6305.946189S (0.07 days)
    - Idle for 278.16%, PT12232229.503484S (141.58 days)
    - Invalid for 0.00%, PT105.205117S (0.00 days)
    - Reserved for 0.73%, PT32206.83027S (0.37 days)
    - StartedOrCrashed for 8.36%, PT367673.212093S (4.26 days)
### ci2.servo.org.log
Over the last PT3237622.61303S (37.47 days) of uptime:
- 1114 runners in profile servo-macos13:
    - Busy for 0.85%, PT27582.546489S (0.32 days)
    - DoneOrUnregistered for 0.01%, PT287.674168S (0.00 days)
    - Idle for 4.66%, PT150838.538644S (1.75 days)
    - Invalid for 0.00%, PT6.131445S (0.00 days)
    - Reserved for 0.04%, PT1419.986452S (0.02 days)
    - StartedOrCrashed for 3.73%, PT120736.699154S (1.40 days)
- 18 runners in profile servo-macos14:
    - Busy for 0.01%, PT263.49627S (0.00 days)
    - DoneOrUnregistered for 0.00%, PT6.093588S (0.00 days)
    - Idle for 90.06%, PT2915796.337922S (33.75 days)
    - Reserved for 0.00%, PT33.196788S (0.00 days)
    - StartedOrCrashed for 0.03%, PT897.856105S (0.01 days)
- 4014 runners in profile servo-ubuntu2204:
    - Busy for 12.70%, PT411046.302011S (4.76 days)
    - DoneOrUnregistered for 0.16%, PT5288.134224S (0.06 days)
    - Idle for 273.15%, PT8843489.77199S (102.36 days)
    - Invalid for 0.04%, PT1179.552101S (0.01 days)
    - Reserved for 0.85%, PT27497.625377S (0.32 days)
    - StartedOrCrashed for 11.36%, PT367697.771998S (4.26 days)
### ci3.servo.org.log
Over the last PT3888098.542751S (45.00 days) of uptime:
- 2 runners in profile servo-ubuntu2204:
    - Idle for 0.12%, PT4540.294826S (0.05 days)
    - StartedOrCrashed for 0.00%, PT54.072544S (0.00 days)
- 1240 runners in profile servo-ubuntu2204-bench:
    - Busy for 0.70%, PT27260.561535S (0.32 days)
    - DoneOrUnregistered for 0.02%, PT641.931572S (0.01 days)
    - Idle for 95.15%, PT3699457.693178S (42.82 days)
    - Invalid for 0.00%, PT6.176017S (0.00 days)
    - Reserved for 0.09%, PT3487.732688S (0.04 days)
    - StartedOrCrashed for 3.45%, PT134045.426597S (1.55 days)
### ci4.servo.org.log
Over the last PT3888116.065537S (45.00 days) of uptime:
- 2 runners in profile servo-ubuntu2204:
    - Idle for 0.12%, PT4588.370027S (0.05 days)
    - StartedOrCrashed for 0.00%, PT51.7589S (0.00 days)
- 1216 runners in profile servo-ubuntu2204-bench:
    - Busy for 0.62%, PT24127.762173S (0.28 days)
    - DoneOrUnregistered for 0.01%, PT558.120552S (0.01 days)
    - Idle for 95.26%, PT3703732.108603S (42.87 days)
    - Reserved for 0.08%, PT3112.286828S (0.04 days)
    - StartedOrCrashed for 3.44%, PT133760.751399S (1.55 days)
### Monthly usage (per month of 30 days)
Runner hours spent in Busy, scaled to 30 days:
- servo-macos13: PT458064.909777884S (5.30 days)
- servo-macos14: PT210.951804293S (0.00 days)
- servo-ubuntu2204: PT861374.153848706S (9.97 days)
- servo-ubuntu2204-bench: PT34257.94170027S (0.40 days)
- servo-windows10: PT766371.678097479S (8.87 days)
### Equivalent spend (per month of 30 days)
NOTE: this doesn’t even consider the speedup vs free runners!
- servo-macos13:
    - Namespace macOS arm64 5cpu:
      5.30 days/month × 91.53 EUR/day = 485.28 EUR/month
    - WarpBuild macOS arm64 6cpu:
      5.30 days/month × 97.64 EUR/day = 517.63 EUR/month
    - GitHub macOS arm64 5cpu:
      5.30 days/month × 195.27 EUR/day = 1035.26 EUR/month
- servo-ubuntu2204:
    - Namespace Linux x64 8cpu:
      9.97 days/month × 14.65 EUR/day = 146.01 EUR/month
    - WarpBuild Linux arm64 8cpu:
      9.97 days/month × 14.65 EUR/day = 146.01 EUR/month
    - WarpBuild Linux x64 8cpu:
      9.97 days/month × 19.53 EUR/day = 194.68 EUR/month
    - GitHub Linux x64 8cpu:
      9.97 days/month × 39.05 EUR/day = 389.35 EUR/month
- servo-windows10:
    - Namespace Windows x64 8cpu:
      8.87 days/month × 29.29 EUR/day = 259.81 EUR/month
    - WarpBuild Linux x64 8cpu:
      8.87 days/month × 39.05 EUR/day = 346.41 EUR/month
    - GitHub Windows x64 8cpu:
      8.87 days/month × 78.11 EUR/day = 692.82 EUR/month

## Past reports

- [2025-09-24](2025-09-24.md)
