#[macro_export]
macro_rules! define_filtered_qmp_stream {
    ($stream_name:ident, $variant:ident, $item:ty) => {
        pub struct $stream_name<S>
        where
            S: ::tokio::io::AsyncRead + ::core::marker::Unpin + Send + 'static,
        {
            inner: ::std::pin::Pin<
                ::std::boxed::Box<dyn ::futures::stream::Stream<Item = $item> + Send>,
            >,
            _marker: std::marker::PhantomData<S>,
        }

        impl<S> $stream_name<S>
        where
            S: ::tokio::io::AsyncRead + ::core::marker::Unpin + Send + 'static,
        {
            pub fn from_message_stream(
                stream: crate::qmp::streams::message_stream::QmpMessageStream<S>,
            ) -> Self {
                use ::futures::StreamExt as _;
                let filtered = stream.filter_map(|msg| async move {
                    match msg {
                        crate::qmp::messages::QmpMessage::$variant(v) => Some(v),
                        _ => None,
                    }
                });
                Self {
                    inner: ::std::boxed::Box::pin(filtered),
                    _marker: std::marker::PhantomData,
                }
            }

            pub fn from_reader(reader: S, cancel: ::tokio_util::sync::CancellationToken) -> Self {
                let base =
                    crate::qmp::streams::message_stream::QmpMessageStream::new(reader, cancel);
                Self::from_message_stream(base)
            }
        }

        impl<S> ::futures::stream::Stream for $stream_name<S>
        where
            S: ::tokio::io::AsyncRead + ::core::marker::Unpin + Send + 'static,
        {
            type Item = $item;

            fn poll_next(
                self: ::core::pin::Pin<&mut Self>,
                cx: &mut ::core::task::Context<'_>,
            ) -> ::core::task::Poll<Option<Self::Item>> {
                /* 内部ストリームに丸投げ */
                self.get_mut().inner.as_mut().poll_next(cx)
            }
        }

        impl<S> ::core::marker::Unpin for $stream_name<S> where
            S: ::tokio::io::AsyncRead + ::core::marker::Unpin + Send + 'static
        {
        }
    };
}
