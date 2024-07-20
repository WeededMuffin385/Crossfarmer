import { useEffect, useState } from "react";
import Creatures from "./creatures/creatures";
import Inventory from "./inventory/inventory";
import Quests from "./quests/quests";
import { ApplicationState } from "../application/application";
import './gameplay.css'
import Chat from "./chat/chat";



enum GameplayState {
    Creatures,
    Inventory,
    Quests,
}

type Props = {
    setApplicationState: (applicationState: ApplicationState) => void,
    token: string,
}


const Gameplay: React.FC<Props> = (props) => {
    const [gameplayState, setGameplayState] = useState<GameplayState>(GameplayState.Creatures);
    const [balance, setBalance] = useState<number>(0);

    const updateBalance = () => {
        fetch('http://' + window.location.hostname + ':8080/api/gameplay/accounts/balance', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                token: props.token
            }),
        }).then((response) => {
            response.json().then((data) => {
                setBalance(data.balance);
            });
        });
    }

    useEffect(() => {
        const balance_interval = setInterval(() => {
            updateBalance();
        }, 1000);
    }, [updateBalance]);


    const RenderState = () => {
        switch (gameplayState) {
            case GameplayState.Creatures:
                return <Creatures token={props.token} setApplicationState={props.setApplicationState}/>
            case GameplayState.Inventory:
                return <Inventory/>
            case GameplayState.Quests:
                return <Quests/>
        }
    }





    return (
        <div className='Gameplay'>
            <div className='Top'>
                <div className='Bar'>
                    <img src={require('../assets/images/icons/coin.png')}/>
                    {balance}
                </div>
                <div className='Bar'>
                    <img src={require('../assets/images/icons/gemRed.png')}/>
                    Premium
                </div>
                <div className='Bar'>
                    <img src={require('../assets/images/icons/scroll.png')}/>
                    Experience
                </div>
            </div>

            <div className='Bottom'>
                <div className='Center'>
                    <RenderState/>
                </div>

                <div className='Right'>
                    <div className='Buttons'>
                        <button onClick={() => {setGameplayState(GameplayState.Creatures)}}>Creatures</button>
                        <button onClick={() => {setGameplayState(GameplayState.Inventory)}}>Inventory</button>
                        <button onClick={() => {setGameplayState(GameplayState.Quests)}}>Quests</button>
                    </div>

                    <Chat token={props.token}/>
                </div>
            </div>
        </div>
    )
}



export default Gameplay;