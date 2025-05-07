---
title: debootstrap guide
date: 2024-08-26
toc: true
draft: false
tags:
  - linux
  - guides
---

## 1. what are the benefits over the standard debian installer?

by installing debian manually using debootstrap you have full and absolute control over everything before your first boot.
for **arch** and **gentoo** users, this installation method may feel more familiar.


likewise, a lot of the information available in the [arch wiki](https://wiki.archlinux.org/title/Main_page)
for setting things up applies just as well to installing debian this way.

---

## 2. what are the drawbacks over standard debian (& derivatives) installers?

a lot of the draw of Linux over other operating systems is deeper control of the system.
of course, having deeper control also comes with less convenience; less is done for you, and as such, you need to learn more to have things done the way you want.

installing a debian system via debootstrap will also almost always take longer at first, but might be faster once you become comfortable.

---

## 3. how do i get started?

for installing debian via debootstrap i, ironically, recommend using the [arch installation medium](https://archlinux.org/download/).

the arch installation medium comes with some useful tools that we will be using and has lots of support for things like wifi adapters.

you can burn the downloaded .iso onto a flash drive (USB drive) using an application such as [rufus](https://rufus.ie/en/).

once you have your drive, plug it into whatever you want to install debian onto, and boot into it from the BIOS.
how you get into your BIOS and select a boot drive is specific to your motherboard, but mashing F12 and/or Delete on startup will probably bring you to the right place.

this guide goes over installing debian. for a detailed guide on setting up an encrypted Ubuntu installation you can see [this post](https://semjonov.de/posts/2021-09/minimal-ubuntu-installation-with-debootstrap/) by Anton.

---

## 4. internet connection

now that you're in the arch installation medium, you need to verify that you have an internet connection.
**it's recommended you use an ethernet connection instead of wifi since it vastly simplifies this process!**


you can try this to verify your connection:

```bash
root@archiso ~ # ping 8.8.8.8
```

this will attempt to ping google. **if this succeeds, skip this section.** see the end of the section for a successful ping.


if you're not using ethernet, there's a good chance you'll get something like:

```bash
ping: connect: Network is unreachable
```

this means that you need to connect to wifi.

you will likely need to unblock some required operations first:

```bash
root@archiso ~ # rfkill unblock all
```

then, you can open **iwctl**, a command-line tool for controlling wifi:

```bash
root@archiso ~ # iwctl
NetworkConfigurationEnabled: disabled
StateDirectory: /var/lib/iwd
Version: 2.17
[iwd]#
```

you can list your wireless devices using **device list**:

```bash
[iwd]# device list
```

there is a good chance you'll have one named **wlan0**. enable it like so:

```bash
[iwd]# device wlan0 set-property Powered on
```

this should put it to station mode. you can then scan for networks and list them:
```bash
[iwd]# station wlan0 scan
[iwd]# station wlan0 get-networks
```
this should have some familiar names for wifi networks. connect to yours like so:
```bash
[iwd]# station wlan0 connect MYNETWORK
Type the network passphrase for MYNETWORK psk.
Passphrase: ********
[iwd]# exit
```
now you should be able to ping google again and see success:
```bash
root@archiso ~ # ping 8.8.8.8
PING 8.8.8.8 (8.8.8.8) 56(84) bytes of data.
64 bytes from 8.8.8.8: icmp_seq=1 ttl=114 time=13.4ms
# [ ... continues until you hit Ctrl+C ]
^C
--- 8.8.8.8 ping statistics ---
2 packets transmitted, 2 received, 0% packet loss, time 2003ms
```
now that you have a working internet connection, you can continue.

---

## 5. formatting your drive

before you can install anything, you need to partition and format your drive.
[the arch wiki has a good guide for this](https://wiki.archlinux.org/title/Installation_guide#Partition_the_disks).
since you're on the arch installation medium, this guide should work just fine.
**stop once you hit 2 Installation as those instructions are for Arch Linux, not Debian!**

---

## 6. bootstrapping debian

if you've done everything correctly, you should now have your drive partitioned, formatted, and mounted on **/mnt**.
now you'll need to install debootstrap:
```bash
# update your local repositories
root@archiso ~ # pacman -Sy
# install debootstrap
root@archiso ~ # pacman -S debootstrap
```

now is the time to pick your [debian release](https://wiki.debian.org/DebianReleases):

1. [stable](https://wiki.debian.org/DebianStable) -- major updates every 2 years. updates are slow, but the current release is well-maintained
2. [unstable](https://wiki.debian.org/DebianUnstable) -- the most up-to-date (does not mean recent), but also little to no quality assurance
1. [testing](https://wiki.debian.org/DebianTesting) -- the current working version of the next stable release. a week or so behind unstable, some quality assurance

i personally recommend **testing** as it includes some nice new changes that stable may be missing.
once you've decided, you can bootstrap your debian system like so:
```bash
root@archiso ~ # debootstrap --arch amd64 testing /mnt https://deb.debian.org/debian
# . . . . . . . . . . . . . . . . . | . . L you can change this out for your preferred release
# . . . . . . . . . . . . . . . . . L you'll need to swap this out if you're not using an x86_64 processor (e.g. aarch64)
```

now that you have a minimal debian system bootstrapped, we do some early setup and chroot into our new system:

```bash
# generate the fstab: read more at https://wiki.archlinux.org/title/Fstab
root@archiso ~ # genfstab -U /mnt > /mnt/etc/fstab
# copy the hosts file
root@archiso ~ # cp /etc/hosts /mnt/etc/hosts
# enter the installation
root@archiso ~ # arch-chroot /mnt

# set our environment so we can use debian commands easily
root@archiso:/# source /etc/profile
# set our prompt so we're not confused where we are
root@archiso:/# PS1="(chroot) $PS1"

(chroot) root@archiso:/#
```

---

## 7. configuring debian

we will need to update our apt sources list, since it currently only has the release base:
```bash
# install the lsb_release tool
(chroot) root@archiso:/# apt install lsb-release
# set the CODENAME variable for use
(chroot) root@archiso:/# CODENAME=$(lsb_release --codename --short)

# set our apt sources using a heredoc:
(chroot) root@archiso:/# cat > /etc/apt/sources.list << HEREDOC
> deb https://deb.debian.org/debian/ $CODENAME main contrib non-free
> deb-src https://deb.debian.org/debian/ $CODENAME main contrib non-free
> deb https://security.debian.org/debian-security $CODENAME-security main contrib non-free
> deb-src https://security.debian.org/debian-security $CODENAME-security main contrib non-free
> deb https://deb.debian.org/debian/ $CODENAME-updates main contrib non-free
> deb-src https://deb.debian.org/debian/ $CODENAME-updates main contrib non-free
> HEREDOC

# update our apt cache:
(chroot) root@archiso:/# apt update # update our local apt cache
```

to prevent having to type the above in manually yourself, you can copy-paste the below:
```bash
CODENAME=$(lsb_release --codename --short) cat > /etc/apt/sources.list << HEREDOC
deb https://deb.debian.org/debian/ $CODENAME main contrib non-free
deb-src https://deb.debian.org/debian/ $CODENAME main contrib non-free
deb https://security.debian.org/debian-security $CODENAME-security main contrib non-free
deb-src https://security.debian.org/debian-security $CODENAME-security main contrib non-free
deb https://deb.debian.org/debian/ $CODENAME-updates main contrib non-free
deb-src https://deb.debian.org/debian/ $CODENAME-updates main contrib non-free
HEREDOC
```

for an explanation on the apt sources list, check [this article](https://wiki.debian.org/SourcesList) in the debian wiki.

choose your timezone:
```bash
(chroot) root@archiso:/# dpkg-reconfigure tzdata
```
use the arrow keys to navigate and enter to select.

choose your locales:
```bash
(chroot) root@archiso:/# apt install locales
(chroot) root@archiso:/# dpkg-reconfigure locales
```
use the arrow keys to navigate, space to select, and enter to confirm.
for example, scroll down to **en_US.UTF-8 UTF-8**, space to select, and enter to confirm.


set your hostname:
```bash
# debian is here as an example, you can set it to anything you like
(chroot) root@archiso:/# echo "debian" > /etc/hostname
```

install the kernel:
```bash
# use your arrow keys to navigate, and press q to exit 'less'
(chroot) root@archiso:/# apt search linux-image | less
# this is fine as a default if you don't have a preference
(chroot) root@archiso:/# apt install linux-image-amd64
```

now to install the bootloader. for this we'll be using grub2:
```bash
# install grub2 tools and efi for amd64 (or your architecture)
(chroot) root@archiso:/# apt install grub2 grub-efi-amd64-bin
(chroot) root@archiso:/# apt install
# /boot, or your efi directory; /efi is also common
(chroot) root@archiso:/# grub-install --efi-directory=/boot
# update grub with your kernel
(chroot) root@archiso:/# update-grub
```

and finally, to set up your personal user. a lot of stuff will break if you try to use root as your default user:
```bash
# install the 'sudo' application for privilege escalation
(chroot) root@archiso:/# apt install sudo

# replace 'jack' with your preferred username; and set the password!
(chroot) root@archiso:/# useradd -UmG sudo -s /bin/bash jack
(chroot) root@archiso:/# passwd jack

# set up NetworkManager or your preferred networking tool
(chroot) root@archiso:/# apt install network-manager
(chroot) root@archiso:/# systemctl enable NetworkManager

# all done here! time to leave our chroot
(chroot) root@archiso:/# exit

# unmount the drives to avoid corrupting your work, reboot, and enjoy!
root@archiso ~ # umount -R /mnt
root@archiso ~ # reboot
```
you can now continue to install a display manager (e.g. lightdm), desktop environment (e.g. xfce4) or window manager (e.g. i3), and your usual system utilities!

---

## 8. addendum: encrypted swap & root

a lot of users prefer to encrypt their root and swap. this prevents people from seeing their files if, say, you lose the device.
we encrypt swap as well, since data persists in swap, even after shutdown.
it takes a bit more setup, and starts before you partition your drives.
**this is adapted and simplified from arch's [system](https://wiki.archlinux.org/title/Dm-crypt/Encrypting_an_entire_system) and [swap](https://wiki.archlinux.org/title/Dm-crypt/Swap_encryption) encryption guides.**


for this guide,
- /dev/sda1 is the EFI partition
- /dev/sda2 is the root partition

... but, your setup may differ.
we will have swap space as a [file](https://wiki.archlinux.org/title/Swap#Swap_file).
this will be a rapid-fire of things in order.


immediately after partitioning your drives:
```bash
# so, after this...
root@archiso ~ # fdisk [your disk]

root@archiso ~ # cryptsetup -v luksFormat /dev/sda2
# /dev/sda2 becomes /dev/mapper/root
root@archiso ~ # cryptsetup open /dev/sda2 root
# replace fs with your preferred filesystem
root@archiso ~ # mkfs.fs /dev/mapper/root
root@archiso ~ # mkfs.fat -F 32 /dev/sda1
root@archiso ~ # mount /dev/mapper/root /mnt
root@archiso ~ # mount --mkdir /dev/sda1 /mnt/boot

# for a swapfile on btrfs, you'll need to follow this:
root@archiso ~ # btrfs subvolume create /mnt/swap
# replace 4g below with your preferred size
root@archiso ~ # btrfs filesystem mkswapfile --size 4g --uid clear /mnt/swap/swapfile

# ... if you're not on btrfs, you can just create the swapfile using mkswap:
root@archiso ~ # mkswap -U clear --size 4G --file /mnt/swapfile

root@archiso ~ # swapon /mnt/swapfile
root@archiso ~ # pacman -Sy && pacman -S debootstrap
root@archiso ~ # debootstrap --arch amd64 testing /mnt https://deb.debian.org/debian
root@archiso ~ # genfstab -U /mnt > /mnt/etc/fstab

# now we add the partition UUID to your crypttab so Debian knows to prompt your password.

# get the UUID of your LUKS part:
root@archiso ~ # LUKSUUID=$(lsblk -f | perl -n -e'/crypto_LUKS\s+2\s+([a-z0-9\-]+)/ && print "$1\n"')
# you may want to verify it's the correct, single UUID by running:
root@archiso ~ # lsblk -f
root@archiso ~ # echo $LUKSUUID

# once you've verified it's correct:
root@archiso ~ # echo "root UUID=$LUKSUUID none luks" > /mnt/etc/crypttab
root@archiso ~ # arch-chroot /mnt
(chroot) root@archiso:/# source /etc/profile
(chroot) root@archiso:/# apt install linux-image-amd64 grub2 grub-efi-amd64-bin
(chroot) root@archiso:/# apt install cryptsetup-initramfs
(chroot) root@archiso:/# update-initramfs -u
```

your encrypted root is ready, and you can continue the installation as normal [here](#7-configuring-debian).
you can skip installing the kernel and grub as we've already done that.