import { Component } from '@angular/core';
import { BarComponent } from './bar/bar.component';

@Component({
  selector: 'app-root',
  imports: [BarComponent],
  templateUrl: './app.component.html',
})
export class AppComponent {}
