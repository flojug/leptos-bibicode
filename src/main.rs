
extern crate bibicode;

use leptos::mount::mount_to_body;
use leptos::prelude::*;

fn main()  {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[derive(Default, Debug, Clone)]
struct NumberWithBase {
    base: bibicode::NumeralSystem,
    valeur: String
}

fn convert(based_number: NumberWithBase, tobase: bibicode::NumeralSystem) -> (String, bool) {

    let frombase = based_number.base.clone();
    let dec = bibicode::NumeralSystem::new_from_tag("dec").unwrap();
    let frombase_to_dec: bibicode::BibiCoder = bibicode::BibiCoder::new(frombase, dec);

    let dec = bibicode::NumeralSystem::new_from_tag("dec").unwrap();
    let dec_to_tobase: bibicode::BibiCoder = bibicode::BibiCoder::new(dec, tobase.clone());

    let mut val = based_number.valeur.clone();
    loop {
        let val_b10 = frombase_to_dec.swap(val.as_str());
        if val_b10.is_ok() {
            let val_bdest = dec_to_tobase.swap(val_b10.unwrap().as_str());
            if val_bdest.is_ok() {
                break (val_bdest.unwrap(), false);
            }
        }
        // if error on the edited field we return the initial value to
        // allow the edition of the end of digit (if multi digit)
        if based_number.base == tobase {
            break (val, true);
        }
        // if error we loop to return a valid value on other fields
        val.pop();
        if val.len() == 0 {
            break (String::from("Error"), true);
        }
    }
}


#[component]
fn App() -> impl IntoView {

    let mut values: Vec<(i32, Vec<String>)> = vec![];
    let lettres = [ "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"];

    for i in 2..=36 {
        values.push((i, vec![]));
        for j in 0..i {
            values[(i-2) as usize].1.push(String::from(lettres[j as usize]));
        }
    }

    let (number, set_number) = signal(NumberWithBase{ base:bibicode::NumeralSystem::new_from_tag("dec").unwrap(), valeur:"42".to_string() });

    let values2 = vec![
        ("Decimal", "dec"),
        ("Hexadécimal", "hex"),
        ("Octal", "oct"),
        ("Binary", "bin"),
        ("Bibicode", "bibi"),
        ("Budu", "budu"),
        ("Base58", "base58"),
    ];

    view! {
        <main>

        <div id="title" class="text-4xl md:text-6xl">
            <p>Multi base converter</p>
        </div>

        <div id="content" class="flex flex-col md:flex-row">

        <div>

            {values2.into_iter()
                .map(|n| view! {
                    <div class="flex flex-row">
                        <span class="text-2xl md:text-4xl w-1/4 md:w-1/4 ">{n.0}</span>
                        <input type="text" class="text-2xl md:text-4xl w-3/4 md:w-3/4 "
                            on:input:target=move |ev| {
                                let frombase = bibicode::NumeralSystem::new_from_tag(n.1).unwrap();
                                set_number.set(NumberWithBase{ base:frombase, valeur:ev.target().value() });
                            }
                            prop:value=move || {
                                let tobase = bibicode::NumeralSystem::new_from_tag(n.1).unwrap();
                                convert(number.get(), tobase).0
                            }
                            class:red=move || {
                                let tobase = bibicode::NumeralSystem::new_from_tag(n.1).unwrap();
                                convert(number.get(), tobase).1
                            }
                        />
                    </div>
                })
                .collect_view()}

        </div>

        <div>

            {values.into_iter()
                .map(|n| {
                    let n2 = n.clone();
                    let n3 = n.clone();
                    let n4 = n.clone();
                    view! {
                        <div class="flex flex-row">
                            <span class="text-2xl md:text-4xl w-1/4 md:w-1/4">Base {n.0}</span>
                            <input type="text" class="text-2xl md:text-4xl w-3/4 md:w-3/4"
                                on:input:target=move |ev| {
                                    let frombase = bibicode::NumeralSystem::new_from_strings("".to_string(), vec![n2.1.clone()]).unwrap();
                                    set_number.set(NumberWithBase{ base:frombase, valeur:ev.target().value() });
                                }
                                prop:value=move || {
                                    let tobase = bibicode::NumeralSystem::new_from_strings("".to_string(), vec![n3.1.clone()]).unwrap();
                                    convert(number.get(), tobase).0
                                }
                                class:red=move || {
                                    let tobase = bibicode::NumeralSystem::new_from_strings("".to_string(), vec![n4.1.clone()]).unwrap();
                                    convert(number.get(), tobase).1
                                }
                            />
                        </div>
                    }
                })
                .collect_view()}

            </div>

        </div>

        </main>
    }
}

