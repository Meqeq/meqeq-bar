import { Component } from '@angular/core';
import { LucideAngularModule, Power } from 'lucide-angular';

@Component({
  selector: 'app-power-menu',
  templateUrl: './power-menu.component.html',
  imports: [LucideAngularModule],
})
export class PowerMenuComponent {
  powerIcon = Power;
}
