import React from 'react'
import logo from './logo.svg';
import './App.css';
import Entrance from "../Entrance/Entrance";

class App extends React.Component {
    constructor(props) {
        super(props);

        this.state = {
            default: true,
        };
    }

    handle_click() {
        console.log("pressed button");
        console.log("showed is " + this.state.counter);

        this.setState({
            counter: this.state.counter + 1,
        });
    }

    //handle_authorization() {
    //    console.log("auth handled");
    //}

    render() {
        return <Entrance/>; //<Entrance onAuthorization = {this.handle_authorization}/>;
    }

    default() {
        return (
            <div className="App">
            <header className="App-header">
                <img src={logo} className="App-logo" alt="logo" />
                <p>
                    Edit {this.state.counter}<code> abc src/App.js</code> and save to reload.
                </p>
                <a
                    className="App-link"
                    href="https://reactjs.org"
                    target="_blank"
                    rel="noopener noreferrer"
                    >
                Learn React
                </a>
            </header>
            </div>
        );
    }
}

export default App;
