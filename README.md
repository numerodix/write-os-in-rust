Code examples from the blog "Writing an OS in Rust (Second Edition)" available
on https://os.phil-opp.com/.

See separate branches per iteration.


## Toolchain version

```bash
$ rustup show
Default host: x86_64-unknown-linux-gnu
rustup home:  /home/user/.rustup

installed toolchains
--------------------

stable-x86_64-unknown-linux-gnu
nightly-x86_64-unknown-linux-gnu (default)

active toolchain
----------------

nightly-x86_64-unknown-linux-gnu (directory override for '/home/user/code/write-os-in-rust')
rustc 1.56.0-nightly (5d6804469 2021-08-30)
```


## PCI device scanning

Default nic:

```
$ qemu-system-x86_64 -drive format=raw,file=target/x86_64-blog_os/debug/bootimage-blog_os.bin -serial stdio
00:00.0 PciDevice { vendor: Known(Intel), device: 0x1237, class: Known(Bridge) }
00:01.0 PciDevice { vendor: Known(Intel), device: 0x7000, class: Known(Bridge) }
00:01.1 PciDevice { vendor: Known(Intel), device: 0x7010, class: Known(MassStorageController) }
00:01.3 PciDevice { vendor: Known(Intel), device: 0x7113, class: Known(Bridge) }
00:02.0 PciDevice { vendor: Unknown(0x1234), device: 0x1111, class: Known(DisplayController) }
00:03.0 PciDevice { vendor: Known(Intel), device: 0x100e, class: Known(NetworkController) }
```

AMD PCnet nic:

```
$ qemu-system-x86_64 -drive format=raw,file=target/x86_64-blog_os/debug/bootimage-blog_os.bin -serial stdio -net nic,model=pcnet
qemu-system-x86_64: warning: hub 0 is not connected to host network
00:00.0 PciDevice { vendor: Known(Intel), device: 0x1237, class: Known(Bridge) }
00:01.0 PciDevice { vendor: Known(Intel), device: 0x7000, class: Known(Bridge) }
00:01.1 PciDevice { vendor: Known(Intel), device: 0x7010, class: Known(MassStorageController) }
00:01.3 PciDevice { vendor: Known(Intel), device: 0x7113, class: Known(Bridge) }
00:02.0 PciDevice { vendor: Unknown(0x1234), device: 0x1111, class: Known(DisplayController) }
00:03.0 PciDevice { vendor: Known(AdvancedMicroDevices), device: 0x2000, class: Known(NetworkController) }
```
