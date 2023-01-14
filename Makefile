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
	@echo "mkdir lkh && \
	mount -t 9p -o trans=virtio host lkh && \
	cd lkh && \
	chmod +x ./optee.sh && \
	chmod +x ./spark/run.sh && \
	cp -f ./ta/8768ae54-0ac3-42e2-ba63-c31f1c74bd8b.ta /lib/optee_armtz/"

all: mkdir dep ta client java
	@echo "${COLOR}\n// ------------------ BEGIN ALL ------------------ //\n${NC}"

qemu:
	@echo "${COLOR}\n// ------------------ BEGIN QEMU ------------------ //\n${NC}"
	# make -C ${OPTEE}/build run OPTEE_RUST_ENABLE=y CFG_TEE_RAM_VA_SIZE=0x00800000 CFG_CORE_ASLR=n GDBSERVER=y QEMU_VIRTFS_ENABLE=y QEMU_USERNET_ENABLE=y
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
        -object rng-random,filename=/dev/urandom,id=rng0 -device virtio-rng-pci,rng=rng0,max-bytes=1024,period=1000 -fsdev local,id=fsdev0,path=/home/huahua/Projects/optee/optee_rust/,security_model=none -device virtio-9p-device,fsdev=fsdev0,mount_tag=host -netdev user,id=vmnic,hostfwd=tcp::12345-:12345 -device virtio-net-device,netdev=vmnic

mkdir:
	@echo "${COLOR}\n// ------------------ BEGIN MKDIR ------------------ //\n${NC}"
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

spark:
# copy spark jars
	@echo "${COLOR}\n// ------------------ BEGIN SPARK ------------------ //\n${NC}"
	cp -rf ${SPARK}/dist ${SPARK_OUT_PATH}
	cp -f ${SPARK}/run.sh ${SPARK_OUT_PATH}


java:
# copy java module interface dependency jars
	@echo "${COLOR}\n// ------------------ BEGIN JAVA ------------------ //\n${NC}"
	@mvn package
	@cp -f ./target/dependency/* ${JAVA_OUT_PATH}
	@cp ./target/obliop-1.0.jar ${JAVA_OUT_PATH}
# copy start shell script
	@cp -f ${OPTEE_SPARK}/optee.sh ${OUT_PATH}
# c-JNI make
	@make -C ${OPTEE_CJNI}
	@cp ${OPTEE_CJNI}/libcjni.so ${JAVA_OUT_PATH}

rust: client ta

client:
	@echo "${COLOR}\n// ------------------ BEGIN CLIENT ------------------ //\n${NC}"
	@make -C ${OPTEE_CLIENT} host
# copy rust shared library
	@cp -f ${OPTEE_CLIENT}/target/aarch64-unknown-linux-gnu/debug/libobliclient.so ${JAVA_OUT_PATH}

ta:
	@echo "${COLOR}\n// ------------------ BEGIN TA ------------------ //\n${NC}"
	@make -s -C ../ta
	install -D ../ta/target/aarch64-unknown-optee-trustzone/release/133af0ca-bdab-11eb-9130-43bf7873bf67.ta -t ../../../out/ta
	@echo "cp -f ./ta/133af0ca-bdab-11eb-9130-43bf7873bf67.ta /lib/optee_armtz/"

local:
	@mvn compile
	@cargo build --manifest-path $(CARGO_TOML) 
	@echo "${COLOR}\n// ------------------ BEGIN LOCAL ------------------ //\n${NC}"
	@java -cp $(DEPENDENCIDES_STR):$(CLASSES) $(MAIN_CLASS)

dep:
	@echo "${COLOR}\n// ------------------ BEGIN DEP ------------------ //\n${NC}"
	mvn dependency:copy-dependencies

clean: 
	@echo "${COLOR}\n// ------------------ BEGIN CLEAN ------------------ //\n${NC}"
	mvn clean
	rm -rf /home/huahua/Projects/rust_optee/optee_rust/out