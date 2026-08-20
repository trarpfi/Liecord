import { useState } from 'react';
import { useNavigate, Link } from 'react-router-dom';
import { toast } from 'react-toastify';

export default function Login() {
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [loading, setLoading] = useState(false);
  const navigate = useNavigate();

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);

    try {
      const response = await fetch('/api/auth/login', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ email, password }),
      });

      if (response.ok) {
        const data = await response.json();
        localStorage.setItem('token', data.token);
        toast.success('Login successful');
        navigate('/app');
      } else {
        toast.error('Invalid credentials');
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
        <h1 className="text-3xl font-bold text-center mb-2">Welcome back!</h1>
        <p className="text-discord-channel-default text-center mb-6">
          We're so excited to see you again!
        </p>

        <form onSubmit={handleSubmit} className="space-y-4">
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
            {loading ? 'Logging in...' : 'Log In'}
          </button>

          <p className="text-sm text-discord-channel-default">
            Need an account?{' '}
            <Link to="/register" className="text-discord-link hover:underline">
              Register
            </Link>
          </p>
        </form>
      </div>
    </div>
  );
}
