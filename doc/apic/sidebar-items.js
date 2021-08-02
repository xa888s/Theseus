initSidebarItems({"constant":[["APIC_SPURIOUS_INTERRUPT_VECTOR",""]],"enum":[["InterruptChip",""],["LapicIpiDestination","The possible destination shorthand values for IPI ICR."]],"fn":[["core_count","Returns the number of processor core (local APICs) that exist on this system."],["get_bsp_id",""],["get_lapics","Returns a reference to the list of LocalApics, one per processor core"],["get_my_apic","Returns a reference to the LocalApic for the currently executing processsor core."],["get_my_apic_id","Returns the APIC ID of the currently executing processor core."],["has_x2apic","Returns true if the machine has support for x2apic"],["init","Initially maps the base APIC MMIO register frames so that we can know which LAPIC (core) we are. This only does something for apic/xapic systems, it does nothing for x2apic systems, as required."],["is_bsp","Returns true if the currently executing processor core is the bootstrap processor,  i.e., the first procesor to run "]],"static":[["INTERRUPT_CHIP","The interrupt chip that is currently configured on this machine.  The default is `InterruptChip::PIC`, but the typical case is `APIC` or `X2APIC`, which will be set once those chips have been initialized."]],"struct":[["ApicRegisters","A structure that offers access to APIC/xAPIC through its I/O registers."],["LocalApic","This structure represents a single APIC in the system, there is one per core. "],["RegisterArray",""]]});