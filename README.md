# Troy

> Troy is a tool like clash. The best way is working on router.

## Usage

Generate config:

```shell
$ troy config --generate
```

Modify and check config:

```shell
$ troy config --test
```

Start:

```shell
$ troy daemon
```

Configure ip for tun device.

Then change DNS server to troy.


## Features

- Protocol
  - trojan
- Forward
  - Tun device
  - FakeDNS
  - Rule
  - GeoIP
- Multi outbound
  - Upstream select
