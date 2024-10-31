let targets = #{
    stm32h743zi: #{
        arch: "arm",
        rust_target: "thumbv7em-none-eabihf",
        probe_chip: "STM32743ZI"
    },

    stm32f401re: #{
        arch: "arm",
        rust_target: "thumbv7em-none-eabihf",
        probe_chip: "STM32F401RETx"
    },

};

let target = variable::get("chip");
let target_properties = targets.get(target);
for key in target_properties.keys() {
    variable::set(key, target_properties.get(key));
}
