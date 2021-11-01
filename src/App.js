import React from "react";
import { BrowserRouter as Router, Switch, Route, Link } from "react-router-dom";

export default function App() {
  return (
    <Router>
      <div>
        <nav>
          <ul>
            <li>
              <Link to="/">Home</Link>
            </li>
            <li>
              <Link to="/test">Test</Link>
            </li>
          </ul>
        </nav>
      </div>
      <Switch>
        <Route path="/test">
          <Test />
        </Route>
        <Route path="/">
          <Index />
        </Route>
      </Switch>
    </Router>
  );
}

function Test() {
  return <h2>test</h2>;
}

function Index() {
  return <h1>Index</h1>;
}
