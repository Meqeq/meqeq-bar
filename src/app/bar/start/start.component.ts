import { Component } from '@angular/core';
import { Grip, LucideAngularModule } from 'lucide-angular';

@Component({
  selector: 'app-start',
  templateUrl: './start.component.html',
  imports: [LucideAngularModule],
})
export class StartComponent {
  readonly startIcon = Grip;
}
