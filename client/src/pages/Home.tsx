import { useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { toast } from 'react-toastify';

export default function Home() {
  const [user, setUser] = useState<any>(null);
  const navigate = useNavigate();

  useEffect(() => {
    const token = localStorage.getItem('token');
    if (!token) {
      navigate('/login');
      return;
    }

    fetch('/api/auth/me', {
      headers: { Authorization: `Bearer ${token}` },
    })
      .then((res) => res.json())
      .then((data) => setUser(data))
      .catch(() => {
        toast.error('Session expired');
        navigate('/login');
      });
  }, [navigate]);

  const handleLogout = () => {
    localStorage.removeItem('token');
    navigate('/login');
  };

  if (!user) {
    return (
      <div className="flex items-center justify-center h-screen">
        <div className="animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-discord-blurple"></div>
      </div>
    );
  }

  return (
    <div className="flex h-screen">
      <div className="w-72 bg-discord-darker flex flex-col">
        <div className="flex-1 p-4">
          <div className="flex items-center space-x-3 mb-6">
            <div className="w-10 h-10 rounded-full bg-discord-blurple flex items-center justify-center text-white font-bold">
              {user.username[0].toUpperCase()}
            </div>
            <div>
              <div className="font-semibold">{user.username}</div>
              <div className="text-xs text-discord-channel-default">Online</div>
            </div>
          </div>

          <div className="space-y-2">
            <div className="text-xs font-bold text-discord-channel-default uppercase mb-2">
              Servers
            </div>
            <div className="text-discord-channel-default text-sm p-3 bg-discord-darkest rounded cursor-pointer hover:bg-discord-dark">
              No servers yet
            </div>
          </div>
        </div>

        <div className="p-4 bg-discord-darkest">
          <button
            onClick={handleLogout}
            className="w-full bg-discord-red text-white py-2 rounded hover:bg-red-600 transition"
          >
            Logout
          </button>
        </div>
      </div>

      <div className="flex-1 bg-discord-dark flex items-center justify-center">
        <div className="text-center">
          <h1 className="text-4xl font-bold mb-4">Welcome to LieCord</h1>
          <p className="text-discord-channel-default mb-8">
            Backend not connected. Start the Rust backend to enable full features.
          </p>
          <div className="space-y-2 text-left max-w-md mx-auto bg-discord-darker p-6 rounded">
            <p className="text-sm text-discord-channel-default">To start backend:</p>
            <code className="block bg-discord-darkest p-3 rounded text-xs">
              cd backend<br />
              cargo run
            </code>
          </div>
        </div>
      </div>
    </div>
  );
}
