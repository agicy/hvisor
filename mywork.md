
# 编译命令
source build/envsetup.sh; lunch rk3588_t-userdebug
./build.sh -AUCKu -d rk3588-evb7-lp4-v10

# Mobile-Ubuntu-Server 上同步 update.img
rsync -avhP -e 'ssh -p 7002' rk3588dev@frps.agicy.cn:/home/rk3588dev/data/rockchip_android13_sdk/rockdev/Image-rk3588_t/update.img ./rk3588_t_update.img

# 把工具加到环境变量里面
export PATH="$PATH:$(realpath out/host/linux-x86/bin)"
export PATH="$PATH:$(realpath out/host/linux-x86/bin)"

rk3588dev@xiuos:~/data/rockchip_android13_sdk/rockdev/Image-rk3588_t/workspace$ unpack_bootimg --boot_img boot.img --out boot_img_out
boot magic: ANDROID!
kernel_size: 33943560
kernel load address: 0x10008000
ramdisk size: 1358748
ramdisk load address: 0x11000000
second bootloader size: 371200
second bootloader load address: 0x10f00000
kernel tags load address: 0x10000100
page size: 2048
os version: 13.0.0
os patch level: 2023-08
boot image header version: 2
product name: 
command line args: console=ttyFIQ0 firmware_class.path=/vendor/etc/firmware init=/init rootwait ro loop.max_part=7 androidboot.console=ttyFIQ0 androidboot.wificountrycode=CN androidboot.hardware=rk30board androidboot.boot_devices=fe2e0000.mmc androidboot.selinux=permissive buildvariant=userdebug
additional command line args: 
recovery dtbo size: 0
recovery dtbo offset: 0x0000000000000000
boot header size: 1660
dtb size: 282418
dtb address: 0x0000000011f00000

rk3588dev@xiuos:~/data/rockchip_android13_sdk/rockdev/Image-rk3588_t/workspace$ file boot_img_out/*
boot_img_out/dtb:     Device Tree Blob version 17, size=282418, boot CPU=0, string block size=23130, DT structure block size=259232
boot_img_out/kernel:  Linux kernel ARM64 boot executable Image, little-endian, 4K pages
boot_img_out/ramdisk: gzip compressed data, from Unix, original size modulo 2^32 2941696
boot_img_out/second:  data

# 把 dtb 反编译成 dts，把 chosen 改成上面的 cmdline，重新编译得到 new_dtb


setenv ipaddr   192.168.255.2
setenv netmask  255.255.255.252
setenv serverip 192.168.255.1
setenv board_dtb_addr       0x00400000
setenv hvisor_addr          0x00500000
setenv kernel_addr          0x40400000
setenv root_linux_dtb_addr  0x08300000
setenv ramdisk_addr         0x0a200000

# 从 TFTP 服务器加载每个文件到对应的内存地址
tftp  ${hvisor_addr}          hvisor.bin
tftp  ${board_dtb_addr}       original.dtb
tftp  ${kernel_addr}          android-kernel
tftp  ${root_linux_dtb_addr}  zone0.dtb
tftp  ${ramdisk_addr}         android-ramdisk


# 从 hvisor_addr 启动
bootm ${hvisor_addr} - ${board_dtb_addr}


setenv ipaddr   192.168.255.2
setenv netmask  255.255.255.252
setenv serverip 192.168.255.1
setenv board_dtb_addr       0x00400000
setenv hvisor_addr          0x00500000
setenv kernel_addr          0x40400000
setenv root_linux_dtb_addr  0x08300000
setenv ramdisk_addr         0x0a200000
tftp  ${hvisor_addr} hvisor.bin ; tftp ${board_dtb_addr} original.dtb ; tftp ${kernel_addr} android-kernel ; tftp ${root_linux_dtb_addr} zone0.dtb ; tftp ${ramdisk_addr} android-ramdisk ; bootm ${hvisor_addr} - ${board_dtb_addr}


```
setenv ipaddr   192.168.255.2
setenv netmask  255.255.255.252
setenv serverip 192.168.255.1
setenv kernel_addr          0x10008000
setenv second_addr          0x10f00000
setenv ramdisk_addr         0x11000000
setenv root_linux_dtb_addr  0x11f00000

tftp ${kernel_addr} android-kernel ; tftp ${second_addr} android-second ; tftp ${root_linux_dtb_addr} zone0.dtb ; tftp ${ramdisk_addr} android-ramdisk ; booti ${kernel_addr} ${ramdisk_addr}:${filesize} ${root_linux_dtb_addr}

```

```
setenv ipaddr   192.168.255.2
setenv netmask  255.255.255.252
setenv serverip 192.168.255.1
setenv kernel_addr          0x40400000
setenv ramdisk_addr         0x0a200000
setenv ramdisk_size         0x0014bb9c
setenv root_linux_dtb_addr  0x08300000
tftp ${kernel_addr} android-kernel ; tftp ${root_linux_dtb_addr} zone0.dtb ; tftp ${ramdisk_addr} android-ramdisk ; booti ${kernel_addr} - ${root_linux_dtb_addr}

```