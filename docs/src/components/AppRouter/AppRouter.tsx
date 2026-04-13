import { Container } from '@mantine/core'
import { lazy, Suspense } from 'react'
import { Route, Routes } from 'react-router'

import { AppLayout, Loading } from '@/components'

import { ErrorBoundary } from '../ErrorBoundary'

const LandingPage = lazy(() => import('@/pages/LandingPage'))
const GuidePage = lazy(() => import('@/pages/GuidePage'))
const ConceptPage = lazy(() => import('@/pages/ConceptPages'))
const ReferencePage = lazy(() => import('@/pages/ReferencePages'))
// const NotFoundPage = lazy(() => import('@/pages/NotFoundPage'))

export const AppRouter = () => (
  <Suspense
    fallback={
      <Container h='100vh'>
        <Loading />
      </Container>
    }
  >
    <ErrorBoundary>
      <Routes>
        <Route element={<AppLayout />}>
          <Route path='/' element={<LandingPage />} />
          <Route path='/concept/:slug' element={<ConceptPage />} />
          <Route path='/guides/:topic' element={<GuidePage />} />
          <Route path='/guides/:topic/:slug' element={<GuidePage />} />
          <Route path='/reference/:topic' element={<ReferencePage />} />
          <Route path='/reference/:topic/:slug' element={<ReferencePage />} />
          {/*<Route path='*' element={<NotFoundPage />} />*/}
        </Route>
      </Routes>
    </ErrorBoundary>
  </Suspense>
)
