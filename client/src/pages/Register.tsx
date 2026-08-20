import { useState } from 'react';
import { useNavigate, Link } from 'react-router-dom';
import { toast } from 'react-toastify';

export default function Register() {
  const [username, setUsername] = useState('');
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [loading, setLoading] = useState(false);
  const navigate = useNavigate();

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);

    try {
      const response = await fetch('/api/auth/register', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ username, email, password }),
      });

      if (response.ok) {
        toast.success('Account created! Please log in.');
        navigate('/login');
      } else {
        const data = await response.json();
        toast.error(data.message || 'Registration failed');
      }
    } catch (error) {
      toast.error('Connection error. Backend not running.');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="min-h-screen flex items-center justify-center bg-discord-blurple">
      <div className="bg-discord-darker p-8 rounded-lg shadow-xl w-full max-w-md">
        <h1 className="text-3xl font-bold text-center mb-2">Create an account</h1>

        <form onSubmit={handleSubmit} className="space-y-4">
          <div>
            <label className="block text-xs font-bold mb-2 text-discord-channel-hover uppercase">
              Username
            </label>
            <input
              type="text"
              value={username}
              onChange={(e) => setUsername(e.target.value)}
              className="w-full bg-discord-darkest text-white px-3 py-2 rounded focus:outline-none focus:ring-2 focus:ring-discord-blurple"
              required
            />
          </div>

          <div>
            <label className="block text-xs font-bold mb-2 text-discord-channel-hover uppercase">
              Email
            </label>
            <input
              type="email"
              value={email}
              onChange={(e) => setEmail(e.target.value)}
              className="w-full bg-discord-darkest text-white px-3 py-2 rounded focus:outline-none focus:ring-2 focus:ring-discord-blurple"
              required
            />
          </div>

          <div>
            <label className="block text-xs font-bold mb-2 text-discord-channel-hover uppercase">
              Password
            </label>
            <input
              type="password"
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              className="w-full bg-discord-darkest text-white px-3 py-2 rounded focus:outline-none focus:ring-2 focus:ring-discord-blurple"
              required
            />
          </div>

          <button
            type="submit"
            disabled={loading}
            className="w-full bg-discord-blurple hover:bg-discord-blurple-hover text-white font-bold py-3 rounded transition disabled:opacity-50"
          >
            {loading ? 'Creating...' : 'Continue'}
          </button>

          <p className="text-sm text-discord-channel-default">
            <Link to="/login" className="text-discord-link hover:underline">
              Already have an account?
            </Link>
          </p>
        </form>
      </div>
    </div>
  );
}
