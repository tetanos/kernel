FROM rustup

RUN apt update
RUN apt install -y binutils nasm grub-pc xorriso make 


