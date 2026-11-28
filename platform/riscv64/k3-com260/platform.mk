
BOOT_PATH        := $(image_dir)/../
HVISOR_RAW_BIN_ABS := $(abspath $(hvisor_bin).tmp)

$(hvisor_bin): elf
	$(OBJCOPY) $(hvisor_elf) --strip-all -O binary $@
