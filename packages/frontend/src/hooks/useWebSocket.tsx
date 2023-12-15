import { useEffect, useState } from "react";

interface WebSocketMessage {
  op: number;
  d?: {
    [key: string]: any;
  };
}

export default ({ url }: { url: string }) => {
  const [socket, setSocket] = useState<WebSocket | null>(null);
  const [ready, setReady] = useState(false);
  const [lastMessage, setLastMessage] = useState<WebSocketMessage | null>(null);

  useEffect(() => {
    let csocket: WebSocket | null = null;
    if (socket) {
        csocket = socket;
    } else {
        csocket = new WebSocket(url);
        setSocket(csocket);
    }

    if (!csocket) return;
    
    csocket.onopen = () => {
      console.log("open");
      setReady(true);
    };

    csocket.onclose = () => {
      console.log("close");
      setReady(false);
    };

    csocket.onmessage = (event) => {
      const message = JSON.parse(event.data);
      // Convert message to WebSocketMessage type (Check if it's a valid message)
      if (message.op && message.d) {
        setLastMessage(message as WebSocketMessage);
      }
    };

    return () => csocket?.close();
  }, [socket]);

  const sendMessage = (message: WebSocketMessage) => {
    if (!socket) return;
    // Check if socket is closed
    if (socket.readyState === 3) {
      setReady(false);
      return;
    }

    socket.send(JSON.stringify(message));
  };

  return { sendMessage, lastMessage, reload: () => setSocket(new WebSocket(url)), ready };
};
