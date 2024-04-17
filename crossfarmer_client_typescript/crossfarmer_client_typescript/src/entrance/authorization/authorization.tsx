import { useState } from 'react';
import {EntranceState} from '../entrance'
import { ApplicationState } from '../../application/application';


type Props = {
    setEntranceState: (next_state: EntranceState) => void,

    setApplicationState: (applicationState: ApplicationState) => void,
    setToken: (token: string) => void,
}

type Response = {
    received: boolean,
    success: boolean,
    error: string,
}

const Authorization: React.FC<Props> = (props) => {
    const [response, setResponse] = useState<Response>({received: false, success: false, error: ''});

    const authorize = () => {
        let mail = document.getElementById("mail") as HTMLInputElement;
        let password = document.getElementById("password") as HTMLInputElement;

        
        fetch('http://' + window.location.hostname + ':8080/api/entrance/authorization', {
            method: 'POST',
            headers: {'Content-Type': 'application/json'},
            body: JSON.stringify({
                mail: mail.value,
                password: password.value,
            }),
        }).then((response) => {
            if (response.ok) {
                response.json().then((data) => {
                    props.setToken(data.token);
                    props.setApplicationState(ApplicationState.Gameplay);
                    setResponse({
                        received: true,
                        success: true,
                        error: "",
                    });
                });
            } else {
                response.json().then((data)=>{
                    setResponse({
                        received: true,
                        success: false,
                        error: data.error,
                    })
                })
            }
        }).catch(error => {
            setResponse({
                received: true,
                success: false,
                error: "Connection lost.",
            })
        });
    }


    if (response.received) {
        return (
            <div>
                <h1>
                    {response.success ? 'Registration succeed' : 'Registration failed: ' + response.error}
                </h1>

                <div className='Result'>
                    <button onClick={() => {setResponse({received: false, success: false, error: ''})}}>Ok</button>
                </div>
            </div>
        )
    }

    return (
        <div>
            <h1>
                Authorization
            </h1>

            <label htmlFor='mail'>Email address</label>
            <input id='mail' type='text'/>

            <br/>

            <label htmlFor='password'>Password</label>
            <input id='password' type='password'/>

            <br/>

            <div className='ButtonContainer'>
                <button className='green' onClick={() => {authorize()}}>Login</button>
                <button className='orange' onClick={() => {props.setEntranceState(EntranceState.Registration)}}>No account</button>
            </div>

            <br/>

            <button className='red' onClick={() => {props.setEntranceState(EntranceState.Recovery)}}>Forgot password</button>
        </div>
    )
}

export default Authorization;