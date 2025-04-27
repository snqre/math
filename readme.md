* Treat prefixed directories as large single modules. These directories must be flat, no nested subdirectories unless you are reexporting these directly at the mod.rs file, ie. a prelude. And these must not contain anymore nested modules. These should also be rare. Keep it strictly as flat as possible.

* If for example you have an enum within an obj_ directory, you can add impls with this convention. enum_ext_my_impl, which keeps everything flat. again nested modules are strictly forbiden.

* Common modules that hold other modules or prefixed directories (items) do not need prefixes, and you can nest those as you like.


## obj_
* mod file must contain the entitity.
* Similar to a "class" will contain a struct, enum, or any implementable entitiy.
* may put globally accesible imports at the mod file
* May contain associated types as ie obj_car::Ford but not tr_ (traits). It may have functions however ie. obj_car::drive, or may have static functions on the object itself obj_car::Car::drive().
* May contain an `Error` and `Result` specific to the object itself.
* Image a Java Class but instead of writing it all in one file, it spans an entire directory.
* Use ext_ to denote files as extensions to the object ie ext_truck.rs which helps to find these at a glance. These may contain structs or other items required related to them, or use private items as each ext_ is a submodule with its own scope, which gives great control.

## tr_
* mod file must contain the trait itself.
* for_ used to denote implementations for this ie. for_i8. Useful for a place to put non native implementations, where you do not own the actual struct, enum, or primitive.


# import
# primary-data - struct/enum
# mod core_ ? associated data
# mod main_ ? a main impl if any
# ie mod add {
    boiler::extend!();

    impl Add for PrimaryData {

    }
}
# boiler::expose!(
    core_,
    main_,
    ...
);