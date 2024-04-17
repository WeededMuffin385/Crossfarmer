import { useEffect, useState } from "react";
import './creatures.css'


type Creature = {id: number, name: string, level: number, health: number};
type Creatures = Creature[];




const Creatures = () => {
    const [creatures, setCreatures] = useState<Creatures>([]);

    useEffect(() => {
        const interval = setInterval(() => {
            updateCreatures();
        }, 1000);


        return () => {
            clearInterval(interval);
        }
    }, []);

    const updateCreatures = () => {
        fetch('http://' + window.location.hostname + ':8080/api/gameplay/creatures/list', {
            method: 'GET',
        }).then((response) => {
            response.json().then((data) => {
                setCreatures(data);
            });
        });
    }

    const attackCreature = (id: number) => {
        fetch('http://' + window.location.hostname + ':8080/api/gameplay/creatures/attack', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                id: id,
            }),
        }).then((response) => {
            console.log('attacked');
            updateCreatures();
        });
    }

    return (
        <div className='Creatures'>
            <table>
                <thead>
                    <tr>
                        <th>Name</th>
                        <th>Level</th>
                        <th>Health</th>
                        <th>Action!</th>
                    </tr>
                </thead>

                {creatures.map((creature) => (
                    <tr>
                        <td>{creature.name}</td>
                        <td>{creature.level}</td>
                        <td>{creature.health}</td>
                        <td className='Action'>
                            <button onClick={()=>{attackCreature(creature.id)}}>fight</button>
                            <button>inspect</button>
                        </td>
                    </tr>
                ))}
            </table>
        </div>
    )
}

export default Creatures;