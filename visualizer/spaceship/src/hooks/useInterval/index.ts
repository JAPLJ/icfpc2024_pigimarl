import { useState, useEffect } from 'react';

export const useInterval = (callback: Function, delay_?: number) => {
  const [delay, setDelay] = useState<number>(delay_ ?? 1000);

  useEffect(() => {
    const intervalId = setInterval(() => callback(), delay);
    return () => clearInterval(intervalId);
  }, [delay, callback]);

  return [delay, setDelay];
}
