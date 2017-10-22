import React, { Component } from 'react';
import logo from './logo.svg';
import './App.css';
import Client from './components/Client.jsx'
import { createStore } from 'redux'
import { Provider } from 'react-redux'
import WSEvent from './reducers/wsbuffer.jsx'

let store = createStore(
	WSEvent, /* preloadedState, */
	window.__REDUX_DEVTOOLS_EXTENSION__ && window.__REDUX_DEVTOOLS_EXTENSION__(),
)

class App extends Component {
  render() {
    return (
    <Provider store={store}>
      <Client />
    </ Provider>
    )
  }
}

export default App;
