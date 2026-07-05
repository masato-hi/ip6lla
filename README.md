# ip6ula

Convert modified EUI-64 based IPv6 unique-local addresses to MAC addresses each other.

## Installation

```shell
go install github.com/masato-hi/ip6ula@latest
```

or download from [releases](https://github.com/masato-hi/ip6ula/releases).

## Usage

```text
Usage:
  ip6ula [flags]

Examples:
ip6ula -u -c fd00::300:5eff:fe90:10ff
ip6ula 01-00-5e-90-10-ff

Flags:
  -c, --colon    display the MAC address in colon-separated
  -h, --help     help for ip6ula
  -u, --upcase   display the address in uppercase
```

## Command examples

```shell
$ ip6ula 01-00-5e-90-10-ff
fd00::300:5eff:fe90:10ff
```

```shell
$ ip6ula fd00::300:5eff:fe90:10ff
01-00-5e-90-10-ff
```

```shell
$ ip6ula -c fd00::300:5eff:fe90:10ff
01:00:5e:90:10:ff
```

```shell
$ ip6ula -cu fd00::300:5eff:fe90:10ff
01:00:5E:90:10:FF
```

## Example using dnsmasq

Set the static IP address of the server that will perform Router Advertisements.

In this example, We describe how to configure networking on a Debian.

```text
# /etc/network/interfaces
iface eno1 inet6 static
    address fd00::300:5eff:fe90:10ff/64
    scope site
    autoconf 1
    accept_ra 2
```

Configure dnsmasq.

```text
# /etc/dnsmasq.conf

# Use Router Advertisement.
enable-ra

# If the router lifetime is set to 0, this server will not be configured as a gateway.
# ra-param=interface,<ra-interval>,<router lifetime> 
ra-param=eth1,300,0 

# Configure the RA options.
#
# constructor:eth1 = Determine the notification addresses based on the address configured on eth1.
# ra-stateless = Stateless address autoconfiguration is performed using Router Advertisement.
# ra-names = The server distribute DNS server information using RDNSS.
dhcp-range=::,constructor:eth1,ra-stateless,ra-names

# The server distribute DNS server information using DHCPv6.
dhcp-option=option6:dns-server,fd00::1
```

Configure the client on the local network to disable IPv6 Privacy Extensions.

```text
# /etc/sysctl.conf
net.ipv6.conf.all.use_tempaddr = 1
```
