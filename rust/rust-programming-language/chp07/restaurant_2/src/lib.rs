fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // super allows referencing in the parent module
        super::deliver_order();
    }

    fn cook_order() {}
}
