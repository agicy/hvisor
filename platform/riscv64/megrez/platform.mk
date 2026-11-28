
# HVISOR ENTRY
HVISOR_ENTRY_PA := 0x80200000
BOOT_PATH := $(image_dir)/../


$(hvisor_bin): elf
	$(OBJCOPY) $(hvisor_elf) --strip-all -O binary $@

mkscr:
	mkimage -A arm -T script -C none -n "Boot Script" -d $(BOOT_PATH)boot.txt $(BOOT_PATH)boot.scr