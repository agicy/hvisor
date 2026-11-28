$(hvisor_bin): elf
	$(OBJCOPY) $(hvisor_elf) --strip-all -O binary $@