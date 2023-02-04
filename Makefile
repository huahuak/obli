# /**
#  * @author kahua.li 
#  * @email moflowerlkh@gmail.com
#  * @date 2023/01/14
#  **/

# // ------------------ optee env ------------------ //
SPARK = /home/huahua/Projects/spark-3.3.1

OPTEE = /home/huahua/Projects/optee
OPTEE_RUST = ${OPTEE}/optee_rust
OPTEE_CLIENT = ${OPTEE_RUST}/examples/obli/obliop/obliclient
OPTEE_SPARK = ${OPTEE_RUST}/examples/obli/obliop
OPTEE_CJNI = ${OPTEE_SPARK}/src/main/java/org/kaihua/obliop/interfaces/cjni

OUT_PATH = ${OPTEE_RUST}/out
JAVA_OUT_PATH = ${OUT_PATH}/java
SPARK_OUT_PATH = ${OUT_PATH}/spark
RES_OUT_PATH = ${OUT_PATH}/res
TA_OUT_PATH = ${OUT_PATH}/ta

UUID = 8768ae54-0ac3-42e2-ba63-c31f1c74bd8b

# // ------------------ java ------------------ //
CLASSES = ${OPTEE_SPARK}/target/classes

EMPTY :=
SPACE := $(EMPTY) $(EMPTY)
DEPENDENCIDES = $(wildcard ${OPTEE_SPARK}/target/dependency/*.jar)
DEPENDENCIDES_STR = $(subst ${SPACE},:,${DEPENDENCIDES})

MAIN_CLASS = org.kaihua.obliop.Main

# // ------------------ rust ------------------ //

CARGO_TOML = ${OPTEE_CLIENT}/Cargo.toml

# // ------------------ shell ------------------ //
SHELL = /usr/bin/zsh
COLOR=\033[0;33m
NC=\033[0m # No Color

help:
	@echo "${COLOR}\n// ------------------ BEGIN HELP ------------------ //\n${NC}"
	@echo "mkdir lkh && mount -t 9p -o trans=virtio host lkh && cd lkh && chmod +x ./optee.sh && chmod +x ./spark/run.sh && cp -f ./ta/${UUID}.ta /lib/optee_armtz/"

qemu:
	@echo "${COLOR}\n// ------------------ BEGIN QEMU ------------------ //\n${NC}"
	@# make -C ${OPTEE}/build run OPTEE_RUST_ENABLE=y CFG_TEE_RAM_VA_SIZE=0x00800000 CFG_CORE_ASLR=n GDBSERVER=y QEMU_VIRTFS_ENABLE=y QEMU_USERNET_ENABLE=y QEMU_VIRTFS_HOST_DIR=/home/huahua/Projects/optee/optee_rust/out
	cd /home/huahua/Projects/optee/build/../out/bin && /home/huahua/Projects/optee/build/../qemu/build/aarch64-softmmu/qemu-system-aarch64 \
        -nographic \
        -serial tcp:localhost:54320 -serial tcp:localhost:54321 \
        -smp 4 \
        -s -S -machine virt,secure=on,mte=off,gic-version=3,virtualization=false \
        -cpu max,pauth-impdef=on \
        -d unimp -semihosting-config enable=on,target=native \
        -m 3G \
        -bios bl1.bin           \
        -initrd rootfs.cpio.gz \
        -kernel Image -no-acpi \
        -append 'console=ttyAMA0,38400 keep_bootcon root=/dev/vda2 ' \
         \
        -object rng-random,filename=/dev/urandom,id=rng0 -device virtio-rng-pci,rng=rng0,max-bytes=1024,period=1000 -fsdev local,id=fsdev0,path=/home/huahua/Projects/optee/optee_rust/out/,security_model=none -device virtio-9p-device,fsdev=fsdev0,mount_tag=host -netdev user,id=vmnic,hostfwd=tcp::12345-:12345 -device virtio-net-device,netdev=vmnic

all: mkdir spark dep java rust 
	@echo "${COLOR}\n// ------------------ END ALL ------------------ //\n${NC}"

mkdir:
	@if [ ! -d "${OUT_PATH}" ]; then\
		mkdir ${OUT_PATH};\
	fi
	@if [ ! -d "${JAVA_OUT_PATH}" ]; then\
		mkdir ${JAVA_OUT_PATH};\
	fi
	@if [ ! -d "${SPARK_OUT_PATH}" ]; then\
		mkdir ${SPARK_OUT_PATH};\
	fi
	@if [ ! -d "${RES_OUT_PATH}" ]; then\
		mkdir ${RES_OUT_PATH};\
		cp -r ./res/* ${RES_OUT_PATH};\
	fi
	@if [ ! -d "${TA_OUT_PATH}" ]; then\
		mkdir ${TA_OUT_PATH};\
		cp -r ./res/* ${TA_OUT_PATH};\
	fi

dep: mkdir
	make -C obliop dep

java: mkdir
	make -C obliop java

rust: proto client ta 

.PHONY: proto
proto:
	make -C proto

client: mkdir
	@echo "${COLOR}\n// ------------------ BEGIN CLIENT ------------------ //\n${NC}"
	make -C ${OPTEE_CLIENT} host
# copy rust shared library
	cp -f ${OPTEE_CLIENT}/target/aarch64-unknown-linux-gnu/debug/libobliclient.so ${JAVA_OUT_PATH}

ta: mkdir
	@echo "${COLOR}\n// ------------------ BEGIN TA ------------------ //\n${NC}"
	make -C ta
	install -D ta/target/aarch64-unknown-optee-trustzone/debug/${UUID}.ta -t ${TA_OUT_PATH}
	@echo "// ------------------ help command ------------------ //"
	@echo "cp -f ./ta/${UUID}.ta /lib/optee_armtz/"

spark: mkdir
# copy spark jars
	@echo "${COLOR}\n// ------------------ BEGIN SPARK ------------------ //\n${NC}"
	cp -rf ${SPARK}/dist ${SPARK_OUT_PATH}
	cp -f ${SPARK}/run.sh ${SPARK_OUT_PATH}

local:
	mvn compile
	cargo build --manifest-path $(CARGO_TOML) 
	@echo "${COLOR}\n// ------------------ BEGIN LOCAL ------------------ //\n${NC}"
	java -cp $(DEPENDENCIDES_STR):$(CLASSES) $(MAIN_CLASS)


clean: 
	@echo "${COLOR}\n// ------------------ BEGIN CLEAN ------------------ //\n${NC}"
	make -C obliop clean
	rm -rf /home/huahua/Projects/optee/optee_rust/out