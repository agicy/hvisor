
# HVISOR ENTRY
HVISOR_ENTRY_PA := 0x80200000

# Make hvisor.bin for hifive-premier-p550
$(hvisor_bin): elf
	$(OBJCOPY) $(hvisor_elf) --strip-all -O binary $@

# Copy hvisor.bin & zone0.dtb to tftp dir, used for u-boot booting.
wsl-cp: dtb all
	cp $(hvisor_bin) /mnt/d/tftp/$(BOARD)/
	cp platform/$(ARCH)/$(BOARD)/image/dts/zone0.dtb /mnt/d/tftp/$(BOARD)/