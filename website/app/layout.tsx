import type { Metadata } from 'next';
import './globals.css';

export const metadata: Metadata = {
  title: 'Nexus Robotics OS 4.2 — Make simple robots capable',
  description: 'A Rust-first adaptive robotics runtime for skills, configurable intelligence, learning, simulation, safety, and connected systems.',
  metadataBase: new URL('https://magnexis.github.io/nexus-robotics'),
  openGraph: {
    title: 'Nexus Robotics OS 4.2',
    description: 'Make simple robots capable. Make capable robots yours.',
    type: 'website',
  },
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  );
}
