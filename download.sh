set -e

curl -v -C - --retry 999 -O https://cdn.kernel.org/pub/linux/kernel/v5.x/linux-5.10.9.tar.xz
tar -xf linux-5.10.9.tar.xz
ln -s -f linux-5.10.9.tar.xz example-01-linux-basic/linux-5.10.9