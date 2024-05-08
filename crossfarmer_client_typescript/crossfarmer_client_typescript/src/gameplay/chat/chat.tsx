import { useEffect, useState, KeyboardEvent } from "react";
import './chat.css';

type Message = {username: string, message: string}

type Props = {
    token: string,
}

const Chat: React.FC<Props> = (props) => {
    const [messages, setMessages] = useState<Message[]>([]);

    useEffect(() => {
        const messages_interval = setInterval(() => {
            updateMessages();
            scrollToBottom();
        }, 1000);

        return () => {
            clearInterval(messages_interval);
        }
    }, []);

    const scrollToBottom = () => {
        let bottom = document.getElementById("bottom") as HTMLInputElement;
        bottom.scrollIntoView({ behavior: "smooth" });
    }

    const sendMessage = () => {
        let message = document.getElementById("message") as HTMLInputElement;

        fetch('http://' + window.location.hostname + ':8080/api/messages/send', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                token: props.token,
                message: message.value,
            }),
        }).then((response) => {
            message.value = '';
        });
    }

    const updateMessages = () => {
        fetch('http://' + window.location.hostname + ':8080/api/messages/list', {
            method: 'GET',
        }).then((response) => {
            response.json().then((data) => {
                setMessages(data);
            });
        });
    }

    const handleEnterKeyDown = (event: KeyboardEvent<HTMLInputElement>) => {
        if (event.key === 'Enter') {
            sendMessage();
        }
    };

    return (
        <div className='Chat'>
            <div className='Messages'>
                {messages.map((message) => (
                    <div>
                        {'[' + message.username + ']' + message.message}
                    </div>
                ))} 
                <div id='bottom'/>
            </div>

            <input id='message' onKeyDown={handleEnterKeyDown}/>
        </div>
    )
}

export default Chat;