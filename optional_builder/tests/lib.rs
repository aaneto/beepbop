#[macro_use]
extern crate optional_builder;

#[cfg(test)]
pub mod tests {
    #[test]
    fn test_with_attr() {
        #[optional_builder]
        #[derive(Default)]
        struct Foo {
            pub a: i32,
            pub b: Option<i32>,
            #[optional_builder(skip)]
            pub c: Option<i32>,
        }

        let foo = Foo::default().with_b(2);
        assert_eq!(foo.b, Some(2));

        let foo = Foo::default().with_b(2).without_b();
        assert_eq!(foo.b, None);

        let foo = Foo::default().without_b();
        assert_eq!(foo.b, None);

        let foo = Foo::default();
        assert_eq!(foo.b, None);
    }

    #[test]
    fn test_many_attrs() {
        #[optional_builder]
        #[derive(Default)]
        struct Foo {
            pub a: i32,
            #[optional_builder(skip)]
            pub b: Option<i32>,
            #[optional_builder(skip)]
            pub c: Option<i32>,
            pub d: Option<String>,
            pub e: Option<u32>,
            #[optional_builder(skip)]
            pub f: Option<f32>,
        }

        let foo = Foo::default().with_e(2u32).with_d("AAAS");
        assert_eq!(foo.e, Some(2));
        assert_eq!(foo.d, Some("AAAS".to_string()));
    }
}
