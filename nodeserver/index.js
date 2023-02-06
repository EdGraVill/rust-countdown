import express from 'express';
import childProcess from 'child_process';

const app = express();

app.get('/countdown', (req, res) => {
  const { time, width = 400, height = 300, format = 'mp4' } = req.query;

  const fileFormat = ['mp4', 'gif', 'webp'].includes(format) ? format : 'mp4';
  const mapContentType = {
    mp4: 'video/mp4',
    gif: 'image/gif',
    webp: 'image/webp',
  };

  const process = childProcess.spawn(
    '/Users/edgravill/code/rust/countdown/target/release/animation',
    ['-c', time, '-w', width, '-a', height, '-o', fileFormat],
  );

  res.set('Content-Type', mapContentType[fileFormat]);
  res.set('Content-Disposition', `inline`);
  process.stdout.pipe(res);
});

app.listen(3000, () => {
  console.log('Server is running on port 3000');
});
