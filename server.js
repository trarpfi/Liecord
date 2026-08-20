require('dotenv').config();
const express = require('express');
const cors = require('cors');
const http = require('http');
const socketIO = require('socket.io');

const app = express();
const server = http.createServer(app);
const io = socketIO(server, {
  cors: {
    origin: "*",
    methods: ["GET", "POST"]
  }
});

app.use(cors());
app.use(express.json());

let users = [];
let messages = [];
let servers = [];

app.get('/', (req, res) => {
  res.json({ message: 'LieCord Backend Running', version: '1.0.0' });
});

app.post('/api/auth/register', async (req, res) => {
  try {
    const { username, email, password } = req.body;
    
    const existingUser = users.find(u => u.email === email || u.username === username);
    if (existingUser) {
      return res.status(400).json({ message: 'User already exists' });
    }

    const user = {
      id: Date.now().toString(),
      username,
      email,
      displayName: username,
      avatar: null,
      status: 'online',
      badges: [],
      nitro: { active: false },
      createdAt: new Date()
    };

    users.push(user);

    const token = 'token_' + user.id;

    res.status(201).json({
      user,
      token
    });
  } catch (error) {
    res.status(500).json({ message: error.message });
  }
});

app.post('/api/auth/login', async (req, res) => {
  try {
    const { email, password } = req.body;

    const user = users.find(u => u.email === email);
    if (!user) {
      return res.status(401).json({ message: 'Invalid credentials' });
    }

    user.status = 'online';

    const token = 'token_' + user.id;

    res.json({
      user,
      token
    });
  } catch (error) {
    res.status(500).json({ message: error.message });
  }
});

app.get('/api/auth/me', (req, res) => {
  const token = req.headers.authorization?.split(' ')[1];
  if (!token) {
    return res.status(401).json({ message: 'Not authorized' });
  }

  const userId = token.replace('token_', '');
  const user = users.find(u => u.id === userId);

  if (!user) {
    return res.status(404).json({ message: 'User not found' });
  }

  res.json(user);
});

app.get('/api/servers', (req, res) => {
  res.json(servers);
});

app.post('/api/servers', (req, res) => {
  const { name, description } = req.body;
  const token = req.headers.authorization?.split(' ')[1];
  const userId = token?.replace('token_', '');

  const server = {
    id: Date.now().toString(),
    name,
    description,
    ownerId: userId,
    channels: [
      {
        id: Date.now().toString() + '_1',
        name: 'general',
        type: 'text'
      }
    ],
    members: [userId],
    createdAt: new Date()
  };

  servers.push(server);
  res.status(201).json(server);
});

app.get('/api/channels/:id/messages', (req, res) => {
  const channelMessages = messages.filter(m => m.channelId === req.params.id);
  res.json(channelMessages);
});

app.post('/api/messages', (req, res) => {
  const { content, channelId } = req.body;
  const token = req.headers.authorization?.split(' ')[1];
  const userId = token?.replace('token_', '');
  const user = users.find(u => u.id === userId);

  const message = {
    id: Date.now().toString(),
    content,
    channelId,
    author: user,
    createdAt: new Date()
  };

  messages.push(message);
  io.to(channelId).emit('message:new', message);

  res.status(201).json(message);
});

app.get('/api/nitro/info', (req, res) => {
  res.json({
    plans: {
      basic: {
        name: 'Nitro Basic',
        price: 2.99,
        features: ['Custom emoji', '50MB uploads', 'HD streaming']
      },
      full: {
        name: 'Nitro',
        price: 9.99,
        features: ['500MB uploads', '4K streaming', '2 Server Boosts']
      }
    }
  });
});

app.post('/api/nitro/subscribe', (req, res) => {
  const token = req.headers.authorization?.split(' ')[1];
  const userId = token?.replace('token_', '');
  const user = users.find(u => u.id === userId);

  if (user) {
    user.nitro = { active: true, tier: req.body.tier };
    user.badges.push('nitro');
  }

  res.json({ message: 'Nitro activated', nitro: user.nitro });
});

io.on('connection', (socket) => {
  console.log('User connected:', socket.id);

  socket.on('channel:join', (channelId) => {
    socket.join(channelId);
    console.log('User joined channel:', channelId);
  });

  socket.on('message:send', (data) => {
    io.to(data.channelId).emit('message:new', data);
  });

  socket.on('typing:start', (data) => {
    socket.to(data.channelId).emit('typing:start', data);
  });

  socket.on('typing:stop', (data) => {
    socket.to(data.channelId).emit('typing:stop', data);
  });

  socket.on('disconnect', () => {
    console.log('User disconnected:', socket.id);
  });
});

const PORT = process.env.PORT || 5000;
server.listen(PORT, () => {
  console.log(`LieCord Backend running on port ${PORT}`);
});
