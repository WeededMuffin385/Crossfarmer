import React from 'react'

class Authorization extends React.Component {
    constructor(props) {
        super(props);

        this.state = {
            mail: '',
            password: '',

            status: {
                received: false,
                result: false,
                error: '',
            }
        }
    }

    render = () => {
        return(
            <div>
                {this.state.status.received ? <this.render_status/> : <this.render_form/>}
            </div>
        );
    }

    render_form = () => {
        return(
            <div>
                <h1>
                    Authorization
                </h1>

                <label for='mail'>Email address</label>
                <input id='mail' type='text'/>

                <br/>

                <label for='password'>Password</label>
                <input id='password' type='text'/>

                <br/>

                <div className='button-container'>
                    <button className='green' onClick={this.try_authorization}>Login</button>
                    <button className='orange' onClick={this.props.move_to_registration}>No account</button>
                </div>

                <br/>

                <button className='red' onClick={this.props.move_to_recovery}>Forgot password</button>
            </div>
        );
    }

    render_status = () => {
        return(
            <div>
                <h1>
                    {this.state.status.result ? 'Authorization succeed' : 'Authorization failed: ' + this.state.status.error}
                </h1>

                <button onClick={this.close_status}>Ok</button>
            </div>
        );
    }

    close_status = () => {
        this.setState({
            status: {
                received: false,
                result: false,
            }
        });
    }

    try_authorization = () => {
        let authorization_data = {
            mail: document.getElementById("mail").value,
            password: document.getElementById("password").value,
        };

        let hostname = window.location.hostname;

        fetch('http://' + hostname + ':8080/api/entrance/authorization', {
            method: 'POST',
            headers: {'Content-Type': 'application/json'},
            body: JSON.stringify(authorization_data),
        }).then((response) => {
            if (response.ok) {
                response.json().then((data) => {
                    this.props.move_to_game(data.token);
                });
            } else {
                response.json().then((data)=>{
                    this.setState({
                        status: {
                            received: true,
                            result: false,
                            error: data.error,
                        }
                    })
                })
            }
        }).catch(error => {
            this.setState({
                status: {
                    received: true,
                    result: false,
                    error: "Connection lost.",
                }
            })
        });
    }
}

export default Authorization;