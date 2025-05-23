# nichijid-rs (nichiji daemon rs)

It's a implementation of "TCP Based Daytime Service" [Daytime Protocol [RFC867]](https://datatracker.ietf.org/doc/html/rfc867).
It replies with a ***Japanese pronunciation*** of the server's local time.

Collaborator: [@kory33](https://github.com/kory33)

> [!NOTE]
> For machine useful time use the Time Protocol (RFC-868) [yanorei32/timed-rs](https://github.com/yanorei32/timed-rs).

## Examples

### Server

```
./nichijid-rs # default port usage (0.0.0.0:13)
./nichijid-rs 0.0.0.0:1313 # specific port usage
```

NOTE: 13 is a well-known port. It may require administrative permission.

### Client

```
$ nc -d localhost 13
ni-sen ni-juu san nen hachi gatsu juu hachi nichi kin youbi juu ji roppun juu san byou
```
