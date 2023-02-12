import { StrictMode, Suspense } from 'react';
import { BrowserRouter } from 'react-router-dom';
import ReactDOM from 'react-dom/client';

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <StrictMode>
    <Suspense fallback={null}>
      <BrowserRouter>
      {/* // TODO  */}
      </BrowserRouter>
    </Suspense>
  </StrictMode>
);