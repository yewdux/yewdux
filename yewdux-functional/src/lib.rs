use std::rc::Rc;

use yew::prelude::*;
use yew_functional::*;
use yewdux::store::Store;
use yewdux::dispatch::Dispatch;

/// This hook allows accessing the state of a store. When the store is modified, a re-render is automatically triggered.
///
/// This function returns the state of the store.
///
/// # Example
/// ```ignore
/// # use yew_functional::function_component;
/// # use yew::prelude::*;
/// use yewdux::use_store;
///
/// #[function_component(UseStore)]
/// fn dispatch() -> Html {
///     let state = use_store::<Store>();
///     
///     // Make sure Dispatch is connected.
///     if let Some(state) = state.as_ref() {
///         html! { <p>{ state.value }</p> }
///     } else {
///         html! {}
///     }
/// }
/// ```
pub fn use_store<T: Store>() -> Rc<Option<Rc<T::Model>>> {
    let (state, set_state) = use_state(|| None);

    // persist the Dispatch across renders
    use_ref(|| {
        Dispatch::<T>::new(Callback::from(move |new_state: Rc<T::Model>| {
            set_state(Some(new_state));
        }))
    });

    state
}

/// This hook allows getting a [`Dispatch`] to the store.
///
/// Do not use the `state` method on the [`Dispatch`]. The dispatch should only be used to create callbacks.
/// The proper way to access the state is via the [`use_store`] hook.
///
/// # Example
/// ```ignore
/// # use yew_functional::function_component;
/// # use yew::prelude::*;
/// use yewdux::use_dispatch;
///
/// #[function_component(UseDispatch)]
/// fn dispatch() -> Html {
///     let dispatch = use_dispatch::<CounterStore>();
///     
///     html! {
///         <button onclick=dispatch.callback(|_| Input::Increment)>{ "Increment" }</button>
///     }
/// }
/// ```
pub fn use_dispatch<T: Store>() -> Rc<Dispatch<T>> {
    // persist the Dispatch across renders
    let (dispatch, _set_dispatch) = use_state(|| Dispatch::<T>::new(Callback::from(move |_| ())));

    dispatch
}