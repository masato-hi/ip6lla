# ip6lla

CLI tool to convert IPv6 link-local addresses and MAC addresses to each other.

## Usage

```text
Usage: ip6lla [OPTIONS] <ADDRESS>

Arguments:
  <ADDRESS>  IPv6 or MAC Address

Options:
  -c             Display colon-separated MAC address
  -u             Display the address in uppercase
  -h, --help     Print help
  -V, --version  Print version
```

## Example

```shell
$ ip6lla 01-00-5e-90-10-ff
fe80::300:5eff:fe90:10ff
```

```shell
$ ip6lla fe80::300:5eff:fe90:10ff
01-00-5e-90-10-ff
```

```shell
$ ip6lla -c fe80::300:5eff:fe90:10ff
01:00:5e:90:10:ff
```

```shell
$ ip6lla -cu fe80::300:5eff:fe90:10ff
01:00:5E:90:10:FF
```
