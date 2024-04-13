import React from 'react'
import './Creatures.css';

function create_creature(name, health, level, id) {
    return {
        id: id,
        name: name,
        level: level,
        health: health,
    };
}

class Creatures extends React.Component {
    constructor(props) {
        super(props);


        this.state = {
            creatures: Array(),
        };
    }

    componentDidMount = () => {
        this.update_creatures();
        this.interval_update_creatures = setInterval(this.update_creatures, 1000);
    }

    componentWillUnmount() {
        clearInterval(this.interval_update_creatures);
    }

    update_creatures = () => {
        let hostname = window.location.hostname;

        fetch('http://' + hostname + ':8080/api/gameplay/creatures/list', {
            method: 'GET',
        }).then((response) => {
            response.json().then((data) => {
                // console.log(data);

                this.setState({
                    creatures: data
                });
            });
        });
    }

    update_counter = () => {
        this.setState({
            counter: this.state.counter + 1,
        })

        console.log('secs: ' + this.state.counter);
    }

    render = () => {
        return(
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

                    {this.state.creatures.map((creature) => (this.render_table_creature(creature)))}
                </table>
            </div>
        )
    }

    render_table_creature = (creature) => {
        return(
            <tr>
                <td>{creature.name}</td>
                <td>{creature.level}</td>
                <td>{creature.health}</td>
                <td><button onClick={()=>{this.attack_creature(creature.id)}}>fight</button></td>
            </tr>
        )
    }

    attack_creature = (id) => {
        let hostname = window.location.hostname;

        let attack_data = {
            id: id,
        };

        fetch('http://' + hostname + ':8080/api/gameplay/creatures/attack', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(attack_data),
        }).then((response) => {
            console.log('attacked');
            this.update_creatures();
        });
    }
}


export default Creatures;