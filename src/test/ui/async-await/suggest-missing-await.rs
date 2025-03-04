// edition:2018

fn take_u32(_x: u32) {}

async fn make_u32() -> u32 {
    22
}

#[allow(unused)]
async fn suggest_await_in_async_fn() {
    let x = make_u32();
    take_u32(x)
    //~^ ERROR mismatched types [E0308]
    //~| HELP consider `await`ing on the `Future`
    //~| SUGGESTION .await
}

async fn dummy() {}

#[allow(unused)]
async fn suggest_await_in_async_fn_return() {
    dummy()
    //~^ ERROR mismatched types [E0308]
    //~| HELP consider using a semicolon here
    //~| HELP consider `await`ing on the `Future`
    //~| SUGGESTION .await
}

#[allow(unused)]
async fn suggest_await_on_if() {
    let _x = if true {
        dummy()
        //~^ HELP consider `await`ing on the `Future`
    } else {
        dummy().await
        //~^ ERROR `if` and `else` have incompatible types [E0308]
    };
}

#[allow(unused)]
async fn suggest_await_on_previous_match_arms() {
    let _x = match 0usize {
        0 => dummy(), //~ HELP consider `await`ing on the `Future`
        1 => dummy(),
        2 => dummy().await,
        //~^ `match` arms have incompatible types [E0308]
    };
}

#[allow(unused)]
async fn suggest_await_on_match_expr() {
    let _x = match dummy() { //~ HELP consider `await`ing on the `Future`
        () => {} //~ ERROR mismatched types [E0308]
    };
}

fn main() {}
