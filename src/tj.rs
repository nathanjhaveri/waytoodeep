use std::{future::Future, pin::Pin, task::Context, task::Poll};

use tracing::info;

pub fn try_join<A, B, AR, BR, E>(a: A, b: B) -> impl Future<Output = Result<(AR, BR), E>>
where
    A: Future<Output = Result<AR, E>>,
    B: Future<Output = Result<BR, E>>,
{
    TryJoin {
        a,
        b,
        a_res: None,
        b_res: None,
    }
}
struct TryJoin<A, B, AR, BR, E>
where
    A: Future<Output = Result<AR, E>>,
    B: Future<Output = Result<BR, E>>,
{
    a: A,
    b: B,

    a_res: Option<AR>,
    b_res: Option<BR>,
}

impl<A, B, AR, BR, E> Future for TryJoin<A, B, AR, BR, E>
where
    A: Future<Output = Result<AR, E>>,
    B: Future<Output = Result<BR, E>>,
{
    type Output = Result<(AR, BR), E>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = unsafe { self.get_unchecked_mut() };
        let (a, b) = unsafe {
            (
                Pin::new_unchecked(&mut this.a),
                Pin::new_unchecked(&mut this.b),
            )
        };

        if this.a_res.is_none() {
            match a.poll(cx) {
                Poll::Ready(a_res) => match a_res {
                    Ok(a) => {
                        info!("a is ready!");
                        this.a_res = Some(a);
                    }
                    Err(a_error) => return Poll::Ready(Err(a_error)),
                },
                Poll::Pending => {
                    info!("a is pending");
                }
            }
        }

        if this.b_res.is_none() {
            match b.poll(cx) {
                Poll::Ready(res) => match res {
                    Ok(b) => {
                        info!("b is ready!");
                        this.b_res = Some(b);
                    }
                    Err(e) => return Poll::Ready(Err(e)),
                },
                Poll::Pending => {
                    info!("b is pending");
                }
            }
        }

        if this.a_res.is_some() && this.b_res.is_some() {
            let a = this.a_res.take().unwrap();
            let b = this.b_res.take().unwrap();
            Poll::Ready(Ok((a, b)))
        } else {
            Poll::Pending
        }

    }
}
