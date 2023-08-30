#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub(super) mod mask_32;
#[cfg(target_arch = "x86_64")]
pub(super) mod mask_64;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub(super) mod vector_128;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub(super) mod vector_256;

#[allow(unused_macros)]
macro_rules! quote_classifier {
    ($name:ident, $core:ident, $size:literal, $mask_ty:ty) => {
        pub(crate) struct $name<'i, I>
        where
            I: InputBlockIterator<'i, $size>,
        {
            iter: I,
            classifier: $core,
            phantom: PhantomData<&'i ()>,
        }

        impl<'i, I> $name<'i, I>
        where
            I: InputBlockIterator<'i, $size>,
        {
            #[inline]
            #[allow(dead_code)]
            pub(crate) fn new(iter: I) -> Self {
                Self {
                    iter,
                    classifier: $core::new(),
                    phantom: PhantomData,
                }
            }

            #[inline]
            #[allow(dead_code)]
            pub(crate) fn resume(
                iter: I,
                first_block: Option<I::Block>,
            ) -> (Self, Option<QuoteClassifiedBlock<I::Block, $mask_ty, $size>>) {
                let mut s = Self {
                    iter,
                    classifier: $core::new(),
                    phantom: PhantomData,
                };

                let block = first_block.map(|b| {
                    // SAFETY: target feature invariant
                    let mask = unsafe { s.classifier.classify(&b) };
                    QuoteClassifiedBlock {
                        block: b,
                        within_quotes_mask: mask,
                    }
                });

                (s, block)
            }
        }

        impl<'i, I> FallibleIterator for $name<'i, I>
        where
            I: InputBlockIterator<'i, $size>,
        {
            type Item = QuoteClassifiedBlock<I::Block, $mask_ty, $size>;
            type Error = InputError;

            fn next(&mut self) -> Result<Option<Self::Item>, Self::Error> {
                match self.iter.next()? {
                    Some(block) => {
                        // SAFETY: target_feature invariant
                        let mask = unsafe { self.classifier.classify(&block) };
                        let classified_block = QuoteClassifiedBlock {
                            block,
                            within_quotes_mask: mask,
                        };
                        Ok(Some(classified_block))
                    }
                    None => Ok(None),
                }
            }
        }

        impl<'i, I> QuoteClassifiedIterator<'i, I, $mask_ty, $size> for $name<'i, I>
        where
            I: InputBlockIterator<'i, $size>,
        {
            fn get_offset(&self) -> usize {
                self.iter.get_offset() - $size
            }

            fn offset(&mut self, count: isize) {
                debug_assert!(count >= 0);
                debug!("Offsetting by {count}");

                if count == 0 {
                    return;
                }

                self.iter.offset(count);
            }

            fn flip_quotes_bit(&mut self) {
                self.classifier.internal_classifier.flip_prev_quote_mask();
            }
        }

        impl<'i, I> InnerIter<I> for $name<'i, I>
        where
            I: InputBlockIterator<'i, $size>,
        {
            fn into_inner(self) -> I {
                self.iter
            }
        }
    };
}

#[allow(unused_imports)]
pub(crate) use quote_classifier;