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

    const RenderState = () => {
        switch (gameplayState) {
            case GameplayState.Creatures:
                return <Creatures setApplicationState={props.setApplicationState}/>
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
                    Currency
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
                    <div className='Content'>
                        <div className='Buttons'>
                            <button>Creatures</button>
                            <button>Inventory</button>
                            <button>Quests</button>
                        </div>

                        <Chat token={props.token}/>
                    </div>
                </div>
            </div>
        </div>
    )
}



export default Gameplay;