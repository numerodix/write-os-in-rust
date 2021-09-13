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


## Ubuntu in qemu

Booting:

```
$ qemu-img create -f qcow2 harddrive.img.qcow2 1T
$ qemu-system-x86_64 -cdrom kubuntu-21.04-desktop-amd64.iso -drive file=harddrive.img.qcow2,format=qcow2 -enable-kvm -m 2G -smp 2 -device pcnet,netdev=net0 -netdev user,id=net0,hostfwd=tcp::5555-:22
```

PCI system:

```
$ lspci
00:03.0 Ethernet controller: Advanced Micro Devices, Inc. [AMD] 79c970 [PCnet32 LANCE] (rev 10)

$ lspci -v
...
00:03.0 Ethernet controller: Advanced Micro Devices, Inc. [AMD] 79c970 [PCnet32 LANCE] (rev 10)
    Physical Slot: 3
    Flags: bus master, medium devsel, latency 0, IRQ 11
    I/O ports at c000 [size=32]
    Memory at feb91000 (32-bit, non-prefetchable) [size=32]
    Expansion ROM at feb0000 [disabled] [size=512K]
    Kernel driver in use: pcnet32
    Kernel modules: pcnet32
```

Kernel messages:

```
pcnet32: PCnet/PCI II 79C970A at 0xc000, 52:54:00:12:34:56 as signed IRQ 11
pcnet32: eth0: registered as PCnet/PCI II 79C970A
pcnet32: 1 cards_found
pcnet32: 0000:00:03.0 ens3: renamed from eth0
pcnet32: 0000:00:03.0 ens3: link up
```

### Network comms

On the guest:

```
$ sudo nc -l 22
```

On the host:

```
nc localhost 5555
```
