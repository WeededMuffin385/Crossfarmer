import { useState } from 'react';
import {EntranceState} from '../entrance'


type Props = {
    setEntranceState: (next_state: EntranceState) => void,
}

type Response = {
    received: boolean,
    success: boolean,
    error: string,
}

const Registration: React.FC<Props> = (props) => {
    const [response, setResponse] = useState<Response>({received: false, success: false, error: ''});

    const register = () => {
        let mail = document.getElementById("mail") as HTMLInputElement;
        let username = document.getElementById("username") as HTMLInputElement;
        let password = document.getElementById("password") as HTMLInputElement;

        fetch('http://' + window.location.hostname + ':8080/api/entrance/registration', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                mail: mail.value,
                username: username.value,
                password: password.value,
            }),
        }).then((response) => {
            if (response.ok) {
                setResponse({
                    received: true,
                    success: true,
                    error: "",
                });
            } else {
                response.json().then((data)=>{
                    setResponse({
                        received: true,
                        success: false,
                        error: data.error,
                    });
                })
            }
        }).catch(error => {
            setResponse({
                received: true,
                success: false,
                error: "Connection lost.",
            });
        });
    }

    if (response.received) {
        return (
            <div>
                <h1>
                    {response.success ? 'Registration succeed' : 'Registration failed: ' + response.error}
                </h1>

                <div className='Result'>
                    <button onClick={() => {setResponse({received: false, success: false, error: ''}); props.setEntranceState(EntranceState.Authorization)}}>Ok</button>
                </div>
            </div>
        )
    }

    return(
        <div>
            <h1>Registration</h1>
            
            <label htmlFor='mail'>Email address</label>
            <input id='mail' type='text' />

            <br/>
        
            <label htmlFor='username'>Username</label>
            <input id='username' type='text' />
            
            <br/>

            <label htmlFor='password'>Password</label>
            <input id='password' type='password' />

            <br/>

            <div className='ButtonContainer'>
                <button className='green' onClick={() => {register()}}>Sign up</button>
                <button className='orange' onClick={() => {props.setEntranceState(EntranceState.Authorization)}}>Already have an account</button>
            </div>
        </div>
    )
}

export default Registration;