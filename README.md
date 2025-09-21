# ip6lla

Convert modified EUI-64 based IPv6 link-local addresses to MAC addresses each other.

## Usage

```text
Usage:
  ip6lla [flags]

Examples:
ip6lla -u -c fe80::300:5eff:fe90:10ff
ip6lla 01-00-5e-90-10-ff

Flags:
  -c, --colon    display the MAC address in colon-separated
  -h, --help     help for ip6lla
  -u, --upcase   display the address in uppercase
```

## Examples

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
