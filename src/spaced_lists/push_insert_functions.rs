macro_rules! push_insert_functions {
    (Node; ($($T:ident)?), $position:ty) => {
        pub fn push(&mut self, spacing: S$(, value: $T)?) -> $position {
            display_unwrap!(self.try_push(spacing$(, value ${ignore($T)})?))
        }

        // cannot fail
        pub fn insert(&mut self, position: S$(, value: $T)?) -> $position {
            self.size += 1;
            Skeleton::<Node, _, _>::insert(self.skeleton.clone(), position, ($(value ${ignore($T)})?)).into()
        }

        pub fn try_push(&mut self, spacing: S$(, value: $T)?) -> Result<$position, PushError> {
            self.size += 1;
            Ok(Skeleton::<Node, _, _>::try_push(self.skeleton.clone(), spacing, ($(value ${ignore($T)})?))?.into())
        }
    };
    ($range_kind:ident; ($($T:ident)?), $position:ty) => {
        paste! {
            pub fn push(&mut self, spacing: S, span: S$(, value: $T)?) -> $position {
                display_unwrap!(self.try_push(spacing, span$(, value ${ignore($T)})?))
            }

            pub fn insert(&mut self, start: S, end: S$(, value: $T)?) -> $position {
                display_unwrap!(self.try_insert(start, end$(, value ${ignore($T)})?))
            }

            pub fn insert_with_span(&mut self, start: S, span: S$(, value: $T)?) -> $position {
                display_unwrap!(self.try_insert_with_span(start, span$(, value ${ignore($T)})?))
            }

            pub fn try_push(&mut self, spacing: S, span: S$(, value: $T)?) -> Result<$position, [< $range_kind PushError >]> {
                self.size += 1;
                Ok(Skeleton::<$range_kind, _, _>::try_push(self.skeleton.clone(), spacing, span, ($(value ${ignore($T)})?))?.into())
            }

            pub fn try_insert(&mut self, start: S, end: S$(, value: $T)?) -> Result<$position, [< $range_kind InsertionError >]> {
                self.try_insert_with_span(start, end - start$(, value ${ignore($T)})?)
            }

            pub fn try_insert_with_span(&mut self, start: S, span: S$(, value: $T)?) -> Result<$position, [< $range_kind InsertionError >]> {
                self.size += 1;
                Ok(Skeleton::<$range_kind, _, _>::try_insert(self.skeleton.clone(), start, span, ($(value ${ignore($T)})?))?.into())
            }
        }
    };
}

pub(super) use push_insert_functions;