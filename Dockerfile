FROM ubuntu:17.10 as builder

RUN apt update
RUN apt install -y binutils nasm grub-pc-bin xorriso make


