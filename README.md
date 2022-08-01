Example j1939 with Rust
===

Experiments in reading j1939 from can bus using socketcan and canparse.

## references
- https://github.com/jmagnuson/canparse
- https://github.com/dankamongmen/canscan
- https://github.com/mbr/socketcan-rs
- https://www.kvaser.com/developer-blog/an-introduction-j1939-and-dbc-files
- https://www.csselectronics.com/pages/j1939-pgn-conversion-tool
- https://www.csselectronics.com/pages/j1939-explained-simple-intro-tutorial
- https://www.pragmaticlinux.com/2021/10/can-communication-on-the-raspberry-pi-with-socketcan/
- https://www.engr.colostate.edu/~jdaily/J1939/tools.html
- https://www.autopi.io/blog/j1939-explained/
- http://socialledge.com/sjsu/index.php/DBC_Format


## sample data
- https://github.com/CSS-Electronics/mdf4-converters
- https://canlogger.csselectronics.com/canedge-getting-started/log-file-tools/

```
mdf2socketcan -i LOG/958D2219/00002501/00002081.MF4 -O /tmp/can
canplayer -v -g 1000 -I /tmp/can/00002081_CAN.log vcan0=can0
```
